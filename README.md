<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/logos/fiskus-dark-mode.svg">
    <img src="docs/logos/fiskus-light-mode.svg" alt="fiskus" width="320">
  </picture>
</p>

Independent reference engine for German income tax (Einkommensteuer). Today it computes the tax on taxable income (zu versteuerndes Einkommen) in the basic allowance (Grundfreibetrag) zone of § 32a EStG, for tax years 2005 to 2026. Documentation starts at [docs/README.md](docs/README.md).

## Disclaimer

* **Independent reference engine. Not official software of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) or ELSTER. Not tax advice.**
* Pre-release. Only the basic allowance zone of § 32a EStG is implemented.

## Performance

Measured with `make bench` (criterion), release build, one thread, on 2026-11-23:

- Machine: Apple M2 Pro (12 cores, 32 GB), macOS 26.6.2, Rust 1.100.0 (pinned in `mise.toml`).
- `assess`, inside the implemented zone: about 199 million calculations per second.
- `assess`, outside the implemented zone (error path): about 300 million calculations per second.
- Peak memory footprint while benchmarking: 1.6 MB (`/usr/bin/time -l`). Process RSS is 19 MB, which also counts shared system libraries.

The benchmark covers only the implemented zone, so these figures will change as more zones are added. They are updated by hand with `make bench`. The method and the history are in [Benchmarks](docs/internal/benchmarks.md).

## Contributing

Start with [Contributing](docs/contributing/index.md). The gates are `make gates`.

## License

AGPL-3.0-only. A commercial license is available on request.
