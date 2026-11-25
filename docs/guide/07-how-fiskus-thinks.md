{{#include assets/icons/principles.svg}}

# How fiskus thinks

This page sets out the conventions behind every figure in fiskus. It is for people reading the output and for agents changing the code.

## The law is data

The tax rules are not in the code. The figures of each year are in `rules/<year>.toml`: the basic allowance, the tariff zones, and the citation, source and archived copy of each one. The build reads the files and compiles them in, so the library never reads a file at runtime.

The calculation is the same for every year. A new year is a new file, not a new branch of the code.

## Money is exact

Amounts are whole cents (`Cents`). Taxable income is rounded down to whole euros before the tariff. The tax is the one fraction of euros that the zone formula gives, rounded down once. Floating point is used only for the rates, which are ratios for display and never feed back into a tax.

## Errors instead of guesses

fiskus returns an error rather than a number it cannot stand behind:

- `NegativeTaxableIncome`: the income is below zero. It is an input error, not a zero tax.
- `OutsideImplementedRange`: the income is above the basic allowance and fiskus has no tariff zones for the year, or the amount is too large to compute.

A caller that reads `Ok` can trust the figure. A caller that reads `Err` has input to fix or a year to wait for.

## Every figure has a source

Each parameter names its citation (the section of the statute), its source (where the figure was read), the date it was retrieved, and an archived copy in `docs/references`. A draft is marked as a draft in its citation. Figures from memory or from a language model are not used.

## Behaviour is tested for every year

Tests check the behaviour of the tariff in every listed year: no tax up to the allowance, tax never falls as income rises, joint tax is twice the tax on half the income, and the zones join within one euro. They do not check fixed numbers per year. Checking the figures against the calculator of the Federal Ministry of Finance is the job of the conformance layer, which is planned work and not yet in fiskus.

## Sample data

Examples, benchmarks and the charts use the latest listed year and inputs scaled to its basic allowance, so they keep working when a year is added. The few figures quoted in these pages are from the latest year's rules, and a guide check fails when they drift from `rules/`.

## For agents

1. Choose the year with `TaxYear::from_year`, or take it from `TaxYear::ALL`.
2. Build a `FilingUnit` with the taxable income in `Cents`.
3. Call `assess` for the tax, or `explain` for the steps, the rates and the source.
4. Handle the errors above. Do not replace an error with zero.

fiskus is an independent reference engine. It is not official BMF or ELSTER software, and it is not tax advice.
