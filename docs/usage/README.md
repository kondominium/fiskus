# Using fiskus

This page is for developers who call fiskus from Rust or from the WebAssembly build. It describes what the crate does today: the tariff of § 32a EStG for VZ 2006 to 2026, all five zones in each year, for single and joint filing. For the formulas, see `docs/formulas/`.

## Status

fiskus is not on crates.io yet. Depend on it by path or git:

```toml
[dependencies]
fiskus = { path = "../fiskus" }
```

## What it computes today

For each of the 21 tax years from 2006 to 2026, `assess` returns the tariff income tax (tarifliche Einkommensteuer) for a taxable income, across all five zones of § 32a EStG. Zone 1 is the basic allowance and carries zero tax. For VZ 2026:

- Single filing (basic tariff, Grundtarif): zero tax up to 12 348 € of taxable income; tax stays at 0 € up to 12 355 €, because it rounds down.
- Joint filing (splitting tariff, Splittingtarif, § 32a Abs. 5 EStG): zero tax up to 24 711 € of joint taxable income.

`explain` returns the same tax plus the zone used, the unrounded tax, the average and marginal rates, and the statute citation, source and archived reference. `tariff_zones` and `basic_allowance_eur` expose the raw tariff. Amounts are exact integer cents; taxable income is rounded down to whole euros before the tariff is applied, as § 32a Abs. 1 Satz 1 EStG requires.

Years before 2006 are not in `TaxYear`. fiskus does not compute taxable income (zvE) itself; you pass it in.

fiskus does not compute taxable income yet. You pass in taxable income (zu versteuerndes Einkommen, zvE) as an amount.

## Quick start

```rust
use fiskus::{assess, Cents, Error, FilingStatus, FilingUnit, TaxYear};

fn main() -> Result<(), Error> {
    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(12_348),
    };

    let assessment = assess(TaxYear::Y2026, &unit)?;
    assert_eq!(assessment.income_tax, Cents::new(0));

    // 30 000 € single: 4 217 € tax.
    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(30_000),
    };
    let assessment = assess(TaxYear::Y2026, &unit)?;
    assert_eq!(assessment.income_tax, Cents::from_euros(4_217));

    // 100 000 € single: 30 864 € tax.
    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(100_000),
    };
    let assessment = assess(TaxYear::Y2026, &unit)?;
    assert_eq!(assessment.income_tax, Cents::from_euros(30_864));

    // 100 000 € joint: 21 096 € tax.
    let unit = FilingUnit {
        filing: FilingStatus::Joint,
        taxable_income: Cents::from_euros(100_000),
    };
    let assessment = assess(TaxYear::Y2026, &unit)?;
    assert_eq!(assessment.income_tax, Cents::from_euros(21_096));

    Ok(())
}
```

## Inputs

`FilingUnit` (filing unit, Veranlagungseinheit) has two fields:

| Field | Type | Meaning |
|---|---|---|
| `filing` | `FilingStatus` | `Single` (single assessment, Einzelveranlagung) or `Joint` (joint assessment, Zusammenveranlagung) |
| `taxable_income` | `Cents` | Taxable income (zu versteuerndes Einkommen), in cents |

Amounts are `Cents`, an exact integer count of cents. Build one from euros with `Cents::from_euros(12_348)`, or from cents with `Cents::new(1_234_800)`. Fractions of a euro are rounded down before the tariff is applied, as § 32a Abs. 1 Satz 1 EStG requires. A value of `12_348.99` € is treated as `12_348` €.

`FilingStatus` selects the tariff:

- `Single`: basic tariff (Grundtarif) on the whole taxable income.
- `Joint`: splitting tariff (Splittingtarif, § 32a Abs. 5 EStG): the tariff is applied to half of the joint taxable income, and the result is doubled.

## Outputs

`assess` returns an `Assessment`:

| Field | Type | Meaning |
|---|---|---|
| `taxable_income` | `Cents` | zvE rounded down to whole euros |
| `income_tax` | `Cents` | Tariff tax (tarifliche Einkommensteuer), whole euros |

Read the amount in euros with `income_tax.whole_euros()`, or in cents with `income_tax.get()`.

`explain` returns an `Explanation`, with the same tax plus the steps behind it:

| Field | Type | Meaning |
|---|---|---|
| `year` | `TaxYear` | Assessment year |
| `filing` | `FilingStatus` | Filing status used |
| `taxable_income_eur` | `i128` | zvE in whole euros |
| `basis_eur` | `i128` | Basis for the tariff (taxable income, or half of it for joint filing) |
| `basic_allowance_eur` | `i128` | Basic allowance (Grundfreibetrag) of the year |
| `zone` | `Option<ZoneStep>` | Zone used, or `None` in the zero zone |
| `unrounded_basis_tax` | `Fraction` | Tax on the basis before rounding down |
| `basis_tax_eur` | `i128` | Tax on the basis, rounded down |
| `income_tax_eur` | `i128` | Income tax of the filing unit |
| `average_rate` | `f64` | Income tax divided by taxable income |
| `marginal_rate` | `f64` | Tax on the next euro of income |
| `citation` | `&'static str` | Statute citation of the year's tariff |
| `source` | `&'static str` | Source the tariff was taken from |
| `reference` | `&'static str` | Archived copy of the source |

`tariff_zones` returns the progressive zones of a year (zones 2 to 5) as `ZoneStep` values:

| Field | Type | Meaning |
|---|---|---|
| `number` | `u8` | Zone number in § 32a Abs. 1 EStG (2 to 5) |
| `from_eur` | `i128` | First euro of the zone |
| `base_eur` | `i128` | Value of `t` is zero at this euro |
| `step_eur` | `i128` | Size of one unit of `t`, in euros |
| `quadratic_h` | `i128` | Coefficient `a` in hundredths of a euro |
| `linear_h` | `i128` | Coefficient `b` in hundredths of a euro |
| `constant_h` | `i128` | Coefficient `c` in hundredths of a euro |

`Fraction` keeps a rational tax value with `numerator` and `denominator`; `floor()` rounds it down to whole euros.

## Functions

```rust
use fiskus::{assess, basic_allowance_eur, explain, tariff_zones, TaxYear};

let unit = fiskus::FilingUnit {
    filing: fiskus::FilingStatus::Single,
    taxable_income: fiskus::Cents::from_euros(30_000),
};

let assessment = assess(TaxYear::Y2026, &unit)?;
let explanation = explain(TaxYear::Y2026, &unit)?;
let zones = tariff_zones(TaxYear::Y2026);
let allowance = basic_allowance_eur(TaxYear::Y2026);
# Ok::<(), fiskus::Error>(())
```

- `assess(year, &unit)` returns the `Assessment`.
- `explain(year, &unit)` returns the `Explanation`.
- `tariff_zones(year)` returns the zones of the year, from zone 2 upward.
- `basic_allowance_eur(year)` returns the basic allowance (Grundfreibetrag) of the year.

## Rates

`explain` returns the average tax rate (`average_rate`) and the marginal tax rate (`marginal_rate`), as fractions of one euro. The average rate is the income tax divided by the taxable income. The marginal rate is the derivative of the zone's formula, i.e. the tax on the next euro of income.

## Errors

`assess` and `explain` return `Result<_, Error>`:

| Error | When |
|---|---|
| `NegativeTaxableIncome` | `taxable_income` is below zero |

`Error` implements `std::error::Error` and `Display`.

```rust
use fiskus::{assess, Cents, Error, FilingStatus, FilingUnit, TaxYear};

let unit = FilingUnit {
    filing: FilingStatus::Single,
    taxable_income: Cents::new(-100),
};

match assess(TaxYear::Y2026, &unit) {
    Ok(assessment) => println!("tax: {} EUR", assessment.income_tax.whole_euros()),
    Err(Error::NegativeTaxableIncome) => println!("taxable income cannot be negative"),
}
```

## Tax year

`TaxYear` lists the years from 2006 to 2026. Each year has its own basic allowance and tariff. Years before 2006 are not listed. `TaxYear::ALL` contains every listed year, oldest first, and `year()` returns the calendar year.

## Not yet in fiskus

See the [roadmap](https://github.com/kondominium/fiskus/blob/main/ROADMAP.md) for what is planned but not implemented yet.

## Disclaimer

fiskus is an independent reference engine. It is not official BMF or ELSTER software and it is not tax advice.