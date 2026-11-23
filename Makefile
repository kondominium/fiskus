.SILENT:
MISE ?= $(shell which mise)

.PHONY: help check build verify coverage docs docs-serve charts run-example-basic_allowance run-example-guide_charts

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "} {name[NR] = $$1; desc[NR] = $$2; if (length($$1) > width) width = length($$1)} END {fmt = "  %-" width "s %s\n"; for (i = 1; i <= NR; i++) printf fmt, name[i], desc[i]}'

check: ## Run fmt, clippy (deny warnings) and tests
	$(MISE) run check

verify: ## Run Kani proof harnesses
	$(MISE) run verify

coverage: ## Line coverage report (informational, writes target/llvm-cov/lcov.info)
	$(MISE) run coverage

docs: ## Build the docs book into target/book
	$(MISE) run docs

docs-serve: ## Serve the docs book locally with live reload
	$(MISE) run docs-serve

charts: ## Regenerate the guide charts in docs/guide/assets/charts
	$(MISE) run charts

build: ## Build the library, examples and tests
	$(MISE) run build

run-example-basic_allowance: ## Run examples/basic_allowance.rs
	$(MISE) run run-example-basic_allowance

run-example-guide_charts: ## Run scripts/guide/charts.rs (writes the SVG charts)
	$(MISE) run run-example-guide_charts
