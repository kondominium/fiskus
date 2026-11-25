.SILENT:
MISE ?= $(shell which mise)

.PHONY: help build lint test-unit test-doc test-api test-guide test-proofs test-all test-coverage gates bench docs docs-serve docs-clean charts run-example-income-tax run-example-explain-tax test-conformance conformance-cases

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*## "} {name[NR] = $$1; desc[NR] = $$2; if (length($$1) > width) width = length($$1)} END {fmt = "  %-" width "s %s\n"; for (i = 1; i <= NR; i++) printf fmt, name[i], desc[i]}'

build: ## Build the library, examples, tests and benches
	$(MISE) run build

lint: ## Format check and clippy, warnings denied
	$(MISE) run lint

test-unit: ## Unit and property tests in src/
	$(MISE) run test-unit

test-doc: ## Doc tests on public items
	$(MISE) run test-doc

test-api: ## Black-box tests of the public API in tests/
	$(MISE) run test-api

test-guide: ## Guide checks and the chart drift check
	$(MISE) run test-guide

test-proofs: ## Kani proof harnesses
	$(MISE) run test-proofs

test-all: ## Every test category, except coverage
	$(MISE) run test-all

test-coverage: ## Line coverage, fails below 90% (writes target/llvm-cov/lcov.info)
	$(MISE) run test-coverage

gates: ## Every gate CI runs: lint, test-all, test-coverage, docs
	$(MISE) run gates

bench: ## Criterion benchmarks in benches/ (writes target/criterion). Measures, does not gate
	$(MISE) run bench

docs: ## Build the docs book into target/book
	$(MISE) run docs

docs-serve: ## Serve the docs book locally with live reload
	$(MISE) run docs-serve

docs-clean: ## Remove the docs book
	$(MISE) run docs-clean

charts: ## Regenerate the guide charts and the generated tariff tables
	$(MISE) run charts

run-example-income-tax: ## Run examples/income_tax.rs
	$(MISE) run run-example-income-tax

run-example-explain-tax: ## Run examples/explain_tax.rs
	$(MISE) run run-example-explain-tax

test-conformance: ## Conformance tests and scoreboard. Years: make test-conformance YEARS=2010,2026 (or YEARS=all). Not part of gates
	CONFORMANCE_YEARS="$(YEARS)" $(MISE) run test-conformance

conformance-cases: ## Fetch BMF results for generated cases: make conformance-cases YEAR=2026 KIND=kinks|samples [COUNT=50] [DRY_RUN=1]
	YEAR=$(YEAR) KIND=$(KIND) COUNT=$(COUNT) DRY_RUN=$(DRY_RUN) $(MISE) run conformance-cases
