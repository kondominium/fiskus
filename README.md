<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/logos/fiskus-dark-mode.svg">
    <img src="docs/logos/fiskus-light-mode.svg" alt="fiskus" width="320">
  </picture>
</p>

<p align="center">
  Independent reference engine for German income tax (Einkommensteuer).<br>
</p>

> Not official software of the Federal Ministry of Finance (BMF) or ELSTER. Not tax advice.

## Getting Started

`fiskus` is a Rust crate. See [Using fiskus](docs/usage/README.md) for the API and a quick start.

## Documentation

- [Guide](docs/guide/README.md): how the tax is worked out, step by step.
- [Using fiskus](docs/usage/README.md): calling the crate from Rust.
- [Formulas](docs/formulas/grundfreibetrag.md): formulas and boundaries, with citations.
- [Internal](docs/internal/build/README.md): build, testing, benchmarks.

## Contributing

Contributions are welcome. Start with [Contributing](docs/contributing/index.md). The gates are `make gates`.

## License

AGPL-3.0-only. A commercial license is available on request: contact@kondominium.app
