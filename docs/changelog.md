# Changelog

This page lists what changed in each fiskus release. The layout follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Changes that users can notice go under `Unreleased` in the same change, and they move into a version when it is released. Groups with no entries are left out.

fiskus has no release yet. Its version is `0.1.0-dev` until 0.1.0 is cut.

## [Unreleased]

## [0.1.0-beta1] - 2026-11-27

### Added

- Basic allowance (Grundfreibetrag) under § 32a Abs. 1 Satz 2 Nr. 1 EStG for every year from 2003 to 2026, each with its citation and archived source.
- Income tax tariff under § 32a Abs. 1 EStG for 2004 to 2026, for single and for joint filing.
- `explain`, which returns the tariff zone, the unrounded tax, the average and marginal rates, and the source of the year's tariff.
- Solidarity surcharge (Solidaritätszuschlag) under § 4 SolZG for 2024 to 2026, with the free threshold of § 3 Abs. 3 SolZG.
- Conformance tests against the income tax calculator of the Federal Ministry of Finance (BMF), with 500 results for each euro year option from 2002 to 2026. Run them with `make test-conformance`. `make gates` runs them too, and a case that differs from the calculator fails the gate, unless it is one of two open problems listed in the test.
- Property tests that run for every listed year, and Kani proofs of the tariff's arithmetic over stated bounds.
- Build-time checks of each rules file, so a bad value fails the build.
- The guide is one page, *How income taxes are calculated in Germany*, with a calendar chart, a history box for the basic allowance and the surcharge, and a Future section on what fiskus does not cover yet.

- Income tax for VZ 2003, with the table step of § 32a Abs. 2 EStG in its 2002 text: the basis is rounded to blocks of 36 € before the zones apply. The BMF calculator's option for 2002 and 2003 agrees with fiskus in all 500 cases.

### Changed

- Functions that return a value now carry `#[must_use]`, so ignoring a result is a compiler warning. Results are unchanged.
- `Explanation` has a new field, `tariff_basis_eur`: the basis after the table step. It equals `basis_eur` for every year except VZ 2003.
- The `reference` link of every rules file, and of each `[solidarity]` table, points at the GitHub repository. These links were on GitLab. The figures are unchanged.
- The guide's basic allowance chart shows one bar per year, with the value on hover, instead of a step line.

### Fixed

- The basic allowance for VZ 2003 is 7 235 €, not 7 426 €. The 7 426 € stage was postponed to VZ 2004 by the Flutopfersolidaritätsgesetz (BGBl. 2002 I S. 3651). No computed tax changes, because the 2003 tariff is not in fiskus yet.

[Unreleased]: https://github.com/kondominium/fiskus/compare/v0.1.0-beta1...HEAD
[0.1.0-beta1]: https://github.com/kondominium/fiskus/releases/tag/v0.1.0-beta1

