.SILENT:
MISE ?= $(shell which mise)

.PHONY: help check test build verify docs docs-serve charts web web-install web-test web-build web-dev run-example-basic_allowance run-example-guide_charts

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "} {name[NR] = $$1; desc[NR] = $$2; if (length($$1) > width) width = length($$1)} END {fmt = "  %-" width "s %s\n"; for (i = 1; i <= NR; i++) printf fmt, name[i], desc[i]}'

check: ## Run fmt, clippy (deny warnings), all tests, doc tests and the chart check
	$(MISE) run check

test: ## Run unit and property tests in src/
	$(MISE) run test

bench: ## Run criterion benchmarks in benches/
	$(MISE) run bench

size: ## Report packaged crate and WebAssembly sizes
	$(MISE) run size

verify: ## Run Kani proof harnesses
	$(MISE) run verify

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

web: ## Build and test the calculator (wasm core, React app)
	$(MISE) run web-test
	$(MISE) run web-build

web-wasm: ## Build the Rust core for the browser
	$(MISE) run web-wasm

web-install: ## Install the calculator's npm dependencies from the lock file
	$(MISE) run web-install

web-test: ## Run the calculator's tests against the wasm core
	$(MISE) run web-test

web-build: ## Build the calculator into docs/calculator/app/dist
	$(MISE) run web-build

web-dev: ## Serve the calculator locally with live reload
	$(MISE) run web-dev

pages: ## Build and bundle docs + calculator into target/book for GitHub Pages
	$(MISE) run pages

