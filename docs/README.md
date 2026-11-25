<p align="center">
  <img src="logos/fiskus-light-mode-on-white.svg" alt="fiskus" width="320">
</p>

**Income tax, worked out in the open.**

fiskus is an independent reference engine for German income tax (Einkommensteuer). It shows each step of the calculation and the code that performs it. It computes the tariff of § 32a EStG for every assessment year that has tariff zones, and the basic allowance (Grundfreibetrag) for every listed year.

> Not official software of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) or ELSTER. Not tax advice.

## Coverage

| Area | Status |
|---|---|
| Tax years | Every year in `TaxYear::ALL`. The list is the source, not this page |
| Implemented | The tariff of § 32a EStG, for single and joint filing, for each year with tariff zones. The basic allowance for every listed year. Average and marginal rates. The solidarity surcharge, with the free threshold of each year that has its statute archived (VZ 2024 to 2026 so far) |
| Not yet in fiskus | Income types, allowances and special expenses, child allowances, widowed filers, tariff exceptions, church tax (Kirchensteuer), the solidarity surcharge for years before VZ 2024, conformance with the BMF calculator |
| Test coverage | At least 90% of lines, enforced by `make test-coverage` |

## Read

| Page | Read it for |
|---|---|
| [Guide](guide/README.md) | How the tax is worked out, one step at a time |
| [Using fiskus](usage/README.md) | Calling the crate from Rust |
| [Formulas](formulas/grundfreibetrag.md) | The formulas and boundaries, with citations |
| [Internal](internal/build/README.md) | How fiskus is built, tested and measured: [Build](internal/build/README.md), [Testing](internal/testing/README.md), [Benchmarks](internal/benchmarks.md) |
| [Contributing](contributing/index.md) | Workflow, code rules, tests, language, guide, design |

## License

AGPL-3.0-only. Commercial license on request: contact@kondominium.app
