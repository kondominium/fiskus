MISE ?= $(shell which mise)

.PHONY: help check

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "} {printf "  %-10s %s\n", $$1, $$2}'

check: ## Run fmt, clippy (deny warnings) and tests
	$(MISE) run check
