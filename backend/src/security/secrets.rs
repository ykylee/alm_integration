use std::fmt::{Display, Formatter};

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub trait SecretCipher: Send + Sync {
    fn decrypt(
        &self,
        ciphertext: &str,
        key_version: Option<&str>,
    ) -> Result<String, SecretCipherError>;
}

#[derive(Debug)]
pub enum SecretCipherError {
    MissingKey(String),
    InvalidCiphertext(String),
    DecryptFailed,
}

impl Display for SecretCipherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingKey(name) => write!(f, "missing secret cipher key: {name}"),
            Self::InvalidCiphertext(message) => write!(f, "invalid ciphertext: {message}"),
            Self::DecryptFailed => write!(f, "failed to decrypt ciphertext"),
        }
    }
}

impl std::error::Error for SecretCipherError {}

pub struct EnvSecretCipher;

impl EnvSecretCipher {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn encrypt(
        &self,
        plaintext: &str,
        key_version: Option<&str>,
    ) -> Result<String, SecretCipherError> {
        let cipher = self.build_cipher(key_version)?;
        let nonce_bytes = &Uuid::new_v4().into_bytes()[..12];
        let nonce = Nonce::from_slice(nonce_bytes);
        let encrypted = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|_| SecretCipherError::DecryptFailed)?;

        let mut payload = Vec::with_capacity(12 + encrypted.len());
        payload.extend_from_slice(nonce_bytes);
        payload.extend_from_slice(&encrypted);

        Ok(format!("enc-v1:{}", STANDARD.encode(payload)))
    }

    fn build_cipher(&self, key_version: Option<&str>) -> Result<Aes256Gcm, SecretCipherError> {
        let key_material = self.resolve_key_material(key_version)?;
        let derived_key = Sha256::digest(key_material.as_bytes());
        Aes256Gcm::new_from_slice(&derived_key)
            .map_err(|_| SecretCipherError::InvalidCiphertext("invalid derived key".to_string()))
    }

    fn resolve_key_material(&self, key_version: Option<&str>) -> Result<String, SecretCipherError> {
        if let Some(version) = key_version {
            let env_name = format!("ALM_BACKEND_SECRET_KEY_{}", sanitize_key_version(version));
            if let Ok(value) = std::env::var(&env_name) {
                return Ok(value);
            }

            return Err(SecretCipherError::MissingKey(env_name));
        }

        std::env::var("ALM_BACKEND_SECRET_KEY")
            .map_err(|_| SecretCipherError::MissingKey("ALM_BACKEND_SECRET_KEY".to_string()))
    }
}

impl SecretCipher for EnvSecretCipher {
    fn decrypt(
        &self,
        ciphertext: &str,
        key_version: Option<&str>,
    ) -> Result<String, SecretCipherError> {
        if let Some(plaintext) = ciphertext.strip_prefix("plain:") {
            return Ok(plaintext.to_string());
        }

        let Some(encoded) = ciphertext.strip_prefix("enc-v1:") else {
            return Ok(ciphertext.to_string());
        };

        let payload = STANDARD
            .decode(encoded)
            .map_err(|error| SecretCipherError::InvalidCiphertext(error.to_string()))?;
        if payload.len() < 13 {
            return Err(SecretCipherError::InvalidCiphertext(
                "ciphertext payload is too short".to_string(),
            ));
        }

        let (nonce_bytes, encrypted) = payload.split_at(12);
        let cipher = self.build_cipher(key_version)?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher
            .decrypt(nonce, encrypted)
            .map_err(|_| SecretCipherError::DecryptFailed)?;

        String::from_utf8(decrypted)
            .map_err(|error| SecretCipherError::InvalidCiphertext(error.to_string()))
    }
}

fn sanitize_key_version(key_version: &str) -> String {
    key_version
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{EnvSecretCipher, SecretCipher};

    #[test]
    fn env_secret_cipher_round_trips_encrypted_payload() {
        unsafe {
            std::env::set_var("ALM_BACKEND_SECRET_KEY_K1", "test-key-material");
        }
        let cipher = EnvSecretCipher::new();
        let encrypted = cipher.encrypt("jira-secret", Some("k1")).expect("encrypts");

        let decrypted = cipher.decrypt(&encrypted, Some("k1")).expect("decrypts");

        assert_eq!(decrypted, "jira-secret");
    }

    #[test]
    fn env_secret_cipher_supports_legacy_plaintext_values() {
        let cipher = EnvSecretCipher::new();

        assert_eq!(
            cipher
                .decrypt("legacy-token", Some("k1"))
                .expect("decrypts"),
            "legacy-token"
        );
        assert_eq!(
            cipher
                .decrypt("plain:legacy-token", Some("k1"))
                .expect("decrypts"),
            "legacy-token"
        );
    }
}
