<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/logos/fiskus-dark-mode.svg">
    <img src="docs/logos/fiskus-light-mode.svg" alt="fiskus" width="320">
  </picture>
</p>

German individual income tax (Einkommensteuer) for the 2026 tax year: zu versteuerndes Einkommen (taxable income), Einkommensteuer (income tax), and the marginal and average rate.

## Disclaimer

* **Independent reference engine. Not official BMF or ELSTER software. Not tax advice.**
* Pre-release. Only the `Grundfreibetrag` zone of `§ 32a EStG` is implemented.

## Roadmap

See [ROADMAP.md](ROADMAP.md).

## Performance

Measured on a single CPU core with `cargo bench` (criterion), release build:

- `assess`: ~29 million tax calculations per second (~34 ns each)
- `explain`: ~31 million calculations per second (~32 ns each)
- Peak resident memory: under 2 MB
- WebAssembly module: 118 KB

The hot path is O(1) with no heap allocations per assessment.

## License

AGPL-3.0-only. A commercial license is available on request.
