PYTHON ?= python3

.PHONY: install-dev run test

install-dev:
	$(PYTHON) -m pip install -e ".[dev]"

run:
	$(PYTHON) -m uvicorn alm_integration_backend.main:app --reload

test:
	$(PYTHON) -m pytest
