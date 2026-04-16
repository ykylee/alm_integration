FROM docker.io/library/rust:1.94-bookworm

ENV PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

WORKDIR /workspace

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev \
  && rm -rf /var/lib/apt/lists/*

CMD ["cargo", "test", "--manifest-path", "backend/Cargo.toml", "--", "--nocapture"]
