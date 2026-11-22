# 0002: Tariff zones in exact integers, gated by year

Status: accepted for the arithmetic. Superseded in part by [0003](0003-tariffs-for-all-years.md): the formulas now live in `rules/<year>.toml` for every year, and the year gate is gone.
Date: 2026-11-22

## Context

§ 32a Abs. 1 EStG has five zones. Four of them use decimal coefficients, such as 914.51, and every zone rounds the tax down to whole euros. A float would make the rounding depend on binary representation, so the code must use exact integers. The Lohnsteuer repository and the withholding program flowchart (Programmablaufplan, PAP 2026) of the Federal Ministry of Finance (BMF) give the same formulas, so they serve as a cross-check.

## Decision

1. **Exact integers.** Each zone is evaluated with `i128` integers and scaled coefficients, and the result is rounded down with `div_euclid`. Formulas are in `docs/formulas/tariff.md`, with the integer form and the overflow bound.
2. **Joint filers.** The tariff is applied to `⌊income / 2⌋` and doubled (§ 32a Abs. 5 EStG). This is the same rounding as the basic allowance boundary, which gives 24 697 € for joint filers.
3. **Gate by year.** Superseded by 0003: every listed year has its own tariff.
4. **Pure core.** `basis_tax_eur` and `tariff_2026_eur` take parameters as arguments, so Kani can prove them without reading TOML.

## Consequences

- `assess` gives the tax for every whole euro in the 2026 tariff. No overflow is possible for `Cents` values, which Kani checks (`tariff_never_panics`).
- The first euros above the allowance pay no tax, because tax rounds down. Single filers pay 0 € up to 12 355 €. Joint filers pay 0 € up to 24 711 €.
- The formula page was written after the code, not before, as `docs/contributing/code.md` asks. This is recorded here so that the order is not repeated.
- A check outside the repository compared every whole euro from 0 to 300 000 € with an independent calculation, for single and joint filers. It matched on all 600 007 values. It is not yet a repository test. Moving it into `make golden` is an open item.

## Open

- The tariff formulas are constants in `src/tariff.rs`, not in `rules/`. Thresholds and coefficients should move to `rules/<year>/` when a second year is added, so that `build.rs` can generate them.
- Golden fixtures from the PAP (plan checkpoint 3) are still to do.
