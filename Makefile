PYTHON ?= python3
CONTAINER_RUNTIME ?= docker
POSTGRES_CONTAINER ?= alm-postgres
POSTGRES_IMAGE ?= docker.io/library/postgres:17
POSTGRES_DB ?= alm
POSTGRES_USER ?= alm
POSTGRES_PASSWORD ?= secret
POSTGRES_PORT ?= 5432
POSTGRES_VOLUME ?= alm-postgres-data
CONTAINER_NETWORK ?= alm-integration-net
PYTHON_TEST_IMAGE ?= alm-python-test
RUST_TEST_IMAGE ?= alm-rust-test
WORKSPACE_DIR ?= /workspace

.PHONY: install-dev run test backend-run backend-test infra-up infra-down infra-logs infra-wait \
	container-build-python-test container-build-rust-test container-test-python \
	container-test-rust container-test container-backend-run

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
	@$(CONTAINER_RUNTIME) network exists $(CONTAINER_NETWORK) >/dev/null 2>&1 || \
		$(CONTAINER_RUNTIME) network create $(CONTAINER_NETWORK) >/dev/null
	@$(CONTAINER_RUNTIME) volume exists $(POSTGRES_VOLUME) >/dev/null 2>&1 || \
		$(CONTAINER_RUNTIME) volume create $(POSTGRES_VOLUME) >/dev/null
	@$(CONTAINER_RUNTIME) rm -f $(POSTGRES_CONTAINER) >/dev/null 2>&1 || true
	$(CONTAINER_RUNTIME) run -d \
		--name $(POSTGRES_CONTAINER) \
		--network $(CONTAINER_NETWORK) \
		-p $(POSTGRES_PORT):5432 \
		-e POSTGRES_DB=$(POSTGRES_DB) \
		-e POSTGRES_USER=$(POSTGRES_USER) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
		-v $(POSTGRES_VOLUME):/var/lib/postgresql/data \
		$(POSTGRES_IMAGE)

infra-down:
	-$(CONTAINER_RUNTIME) rm -f $(POSTGRES_CONTAINER)

infra-logs:
	$(CONTAINER_RUNTIME) logs -f $(POSTGRES_CONTAINER)

infra-wait:
	@until $(CONTAINER_RUNTIME) exec $(POSTGRES_CONTAINER) pg_isready -U $(POSTGRES_USER) -d $(POSTGRES_DB) >/dev/null 2>&1; do \
		sleep 1; \
	done

container-build-python-test:
	$(CONTAINER_RUNTIME) build -f docker/python-test.Dockerfile -t $(PYTHON_TEST_IMAGE) .

container-build-rust-test:
	$(CONTAINER_RUNTIME) build -f docker/rust-test.Dockerfile -t $(RUST_TEST_IMAGE) .

container-test-python: container-build-python-test
	$(CONTAINER_RUNTIME) run --rm \
		-v "$(CURDIR):$(WORKSPACE_DIR):Z" \
		-w $(WORKSPACE_DIR) \
		$(PYTHON_TEST_IMAGE)

container-test-rust: infra-up infra-wait container-build-rust-test
	$(CONTAINER_RUNTIME) run --rm \
		--network $(CONTAINER_NETWORK) \
		--env-file .env.docker.example \
		-v "$(CURDIR):$(WORKSPACE_DIR):Z" \
		-w $(WORKSPACE_DIR) \
		$(RUST_TEST_IMAGE)

container-test: container-test-python container-test-rust

container-backend-run: infra-up infra-wait container-build-rust-test
	$(CONTAINER_RUNTIME) run --rm \
		--name alm-backend-dev \
		--network $(CONTAINER_NETWORK) \
		-p 8080:8080 \
		--env-file .env.docker.example \
		-v "$(CURDIR):$(WORKSPACE_DIR):Z" \
		-w $(WORKSPACE_DIR) \
		$(RUST_TEST_IMAGE) \
		cargo run --manifest-path backend/Cargo.toml
