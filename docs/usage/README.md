# Using fiskus

This page is for developers who call fiskus from Rust. It describes what the crate does today, which is one zone of the tariff. For the formulas, see [Formulas](../formulas/grundfreibetrag.md).

## Status

fiskus is not on crates.io yet. Depend on it by path or git:

```toml
[dependencies]
fiskus = { path = "../fiskus" }
```

## What it computes today

For tax years 2003 to 2026, `assess` returns the income tax (Einkommensteuer) for taxable income (zu versteuerndes Einkommen, zvE) in the basic allowance (Grundfreibetrag) zone of § 32a EStG. In that zone the tax is 0 €. Pass the year as `TaxYear::Y2003` to `TaxYear::Y2026`.

- Single filing, basic tariff (Grundtarif): zero tax up to 12 348 € of taxable income.
- Joint filing, splitting tariff (Splittingtarif, § 32a Abs. 5 EStG): zero tax up to 24 697 € of joint taxable income.

Above these amounts, `assess` returns `Error::OutsideImplementedRange`. It does not estimate the tax. Other tariff zones are not yet in fiskus.

fiskus does not compute taxable income yet. You pass it in as an amount.

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
    Ok(())
}
```

## Inputs

The filing unit (Veranlagungseinheit) is `FilingUnit`. It has two fields:

| Field | Type | Meaning |
|---|---|---|
| `filing` | `FilingStatus` | `Single` (single assessment, Einzelveranlagung) or `Joint` (joint assessment, Zusammenveranlagung) |
| `taxable_income` | `Cents` | Taxable income (zu versteuerndes Einkommen), in cents |

Amounts are `Cents`, an exact integer count of cents. Build one from euros with `Cents::from_euros(12_348)`, or from cents with `Cents::new(1_234_800)`. Fractions of a euro are rounded down before the tariff is applied, as § 32a Abs. 1 Satz 1 EStG requires. An amount of 12 348.99 € is treated as 12 348 €.

## Outputs

`assess` returns an `Assessment`:

| Field | Type | Meaning |
|---|---|---|
| `taxable_income` | `Cents` | Taxable income, rounded down to whole euros |
| `income_tax` | `Cents` | Tariff tax (tarifliche Einkommensteuer), whole euros |

Read the amount in euros with `income_tax.whole_euros()`, or in cents with `income_tax.get()`.

## Errors

`assess` returns `Result<Assessment, Error>`:

| Error | When |
|---|---|
| `NegativeTaxableIncome` | `taxable_income` is below zero |
| `OutsideImplementedRange { taxable_income_eur }` | Above the basic allowance zone (joint: above 24 697 € of joint income) |

`Error` implements `std::error::Error` and `Display`.

```rust
use fiskus::{assess, Cents, Error, FilingStatus, FilingUnit, TaxYear};

let unit = FilingUnit {
    filing: FilingStatus::Single,
    taxable_income: Cents::from_euros(12_349),
};

match assess(TaxYear::Y2026, &unit) {
    Ok(assessment) => println!("tax: {} EUR", assessment.income_tax.whole_euros()),
    Err(Error::OutsideImplementedRange { taxable_income_eur }) => {
        println!("{taxable_income_eur} EUR is above the implemented range");
    }
    Err(other) => println!("{other}"),
}
```

## Tax year

`TaxYear` has one variant per implemented year, `TaxYear::Y2003` to `TaxYear::Y2026`. Do not assume a year outside that range works.

## Not covered yet

- Income tax above the basic allowance zone.
- Taxable income from income (§ 2 EStG), including income-related expenses (Werbungskosten), special expenses (Sonderausgaben) and pension and insurance expenses (Vorsorgeaufwendungen).
- Solidarity surcharge (Solidaritätszuschlag) and church tax (Kirchensteuer).
- Rates: marginal and average rates are not yet in fiskus.

## Disclaimer

fiskus is an independent reference engine. It is not official software of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) or ELSTER, and it is not tax advice.
