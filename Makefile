MISE ?= $(shell which mise)

.PHONY: help check verify docs docs-serve

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "} {printf "  %-10s %s\n", $$1, $$2}'

check: ## Run fmt, clippy (deny warnings) and tests
	$(MISE) run check

verify: ## Run Kani proof harnesses
	$(MISE) run verify

docs: ## Build the docs book into target/book
	$(MISE) run docs

docs-serve: ## Serve the docs book locally with live reload
	$(MISE) run docs-serve
