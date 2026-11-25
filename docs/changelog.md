# Changelog

This page lists what changed in each fiskus release. The layout follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Changes that users can notice go under `Unreleased` in the same change, and they move into a version when it is released. Groups with no entries are left out.

fiskus has no release yet. Its version is `0.1.0-dev` until 0.1.0 is cut.

## [Unreleased]

### Added

- Basic allowance (Grundfreibetrag) under § 32a Abs. 1 Satz 2 Nr. 1 EStG for every year from 2003 to 2026, each with its citation and archived source.
- Income tax tariff under § 32a Abs. 1 EStG for 2004 to 2026, for single and for joint filing. The year 2003 has its basic allowance, but its tariff is not yet in fiskus.
- `explain`, which returns the tariff zone, the unrounded tax, the average and marginal rates, and the source of the year's tariff.
- Solidarity surcharge (Solidaritätszuschlag) under § 4 SolZG for 2024 to 2026, with the free threshold of § 3 Abs. 3 SolZG.
- Conformance tests against the income tax calculator of the Federal Ministry of Finance (BMF), with 500 results for each euro year option from 2002 to 2026. Run them with `make test-conformance`. `make gates` runs them too, but a failing case does not fail the gate yet.
- Property tests that run for every listed year, and Kani proofs of the tariff's arithmetic over stated bounds.
- Build-time checks of each rules file, so a bad value fails the build.

### Changed

- Functions that return a value now carry `#[must_use]`, so ignoring a result is a compiler warning. Results are unchanged.
