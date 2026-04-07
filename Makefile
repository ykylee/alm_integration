PYTHON ?= python3
COMPOSE ?= docker compose

.PHONY: install-dev run test backend-run backend-test infra-up infra-down infra-logs

install-dev:
	$(PYTHON) -m pip install -e ".[dev]"

run:
	$(PYTHON) -m uvicorn alm_integration_backend.main:app --reload

test:
	$(PYTHON) -m pytest

backend-run:
	cargo run --manifest-path backend/Cargo.toml

backend-test:
	cargo test --manifest-path backend/Cargo.toml

infra-up:
	$(COMPOSE) up -d postgres

infra-down:
	$(COMPOSE) down

infra-logs:
	$(COMPOSE) logs -f postgres
