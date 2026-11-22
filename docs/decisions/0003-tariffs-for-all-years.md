# 0003: Tariffs for every year from 2006 to 2026

Status: accepted. Supersedes the year gate in [0002](0002-tariff-exact-integers.md).
Date: 2026-11-22

## Context

Every listed tax year had its basic allowance, but only VZ 2026 had its tariff above it. The tariff zones changed in most years, so a year cannot reuse another year's formulas. The Lohnsteuer repository (https://github.com/MarcelLehmann/Lohnsteuer, Apache-2.0) commits, for each year, the generated Java of the BMF withholding guidance (Programmablaufplan, PAP) and the BMF's XML version of it. We read the tariff method of each year from that Java, checked its coefficients against the text of the BMF PAP for that year, and checked the 2024 to 2026 values against the statute. We copied no code.

## Decision

1. **One tariff per year.** Each `rules/<year>.toml` lists the zones of that year: where each zone starts, the base and step of its variable, and its three coefficients in hundredths of a euro. The same evaluation (`zone_tax_eur`) serves every year, so the data changes and the code does not.
2. **Shape.** Every year has a zero zone, two quadratic zones and two linear zones, except 2006 and 2007. VZ 2006 has no 45 % zone. The 45 % rate starts in VZ 2007.
3. **Sources.** The coefficients for VZ 2006 to 2023 come from the BMF PAP text of each year, through the Lohnsteuer repository's transcription. For VZ 2024 the statute text of BGBl. 2024 I Nr. 386 is the reference, and for 2025 and 2026 the text of BGBl. 2024 I Nr. 449. The PAP 2024 file is a letter without formulas, so the statute is the reference for 2024.
4. **Checks at build time.** `build.rs` rejects a tariff whose first zone does not start at the basic allowance plus one euro, whose zones are not in increasing order, whose reference file is missing, or whose tax jumps by more than one euro at a zone start.
5. **Errors.** `Error::OutsideImplementedRange` is removed. With every listed year implemented, the case cannot occur. The only error left is `NegativeTaxableIncome`.

## Verification

- Every coefficient of every year appears in the text of that year's PAP, or in the statute for 2024 to 2026. Five coefficients appear with trailing zeros dropped (for example `173,1` for 173.10), and they were confirmed by hand.
- The build's continuity check passes for all 21 years.
- The unit tests compare five incomes per year (20 000 €, 40 000 €, 100 000 €, 250 000 € and 300 000 €) with an independent calculation in exact fractions, run on the same tables.
- For VZ 2026, a comparison outside the repository covered every whole euro from 0 € to 300 000 €. All 600 007 values matched.

## Consequences

- `assess` gives the tax for VZ 2006 to 2026 for any income `Cents` can hold, with the basic allowance of that year.
- The formula page `docs/formulas/tariff.md` lists every year's coefficients. The tables are generated from the rule files and must be regenerated when a rule changes.
- The tests loop over `TaxYear::ALL`, so a new year without its tariff fails to build.

## Open

- **VZ 2002 to 2005.** No tariff yet. The Lohnsteuer repository starts at 2006, so these years need their own PAPs.
- **VZ 2014.** The PAP is a draft according to its file name. The statute text, as transcribed on buzer.de, gives the same coefficients.
- **Origin of the XML.** The repository mirrors a BMF XML for each year. That answers plan §3.2 in part. The bmf-steuerrechner.de site was unreachable, so the origin is not yet verified against the BMF.
- **Constants in code.** Coefficients are in `rules/`, but the formula shape (the zone kinds and the joint halving) is in `src/tariff.rs`. A different shape in a future year would need code.
