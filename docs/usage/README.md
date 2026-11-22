# Using fiskus

This page is for developers who call fiskus from Rust. It describes what the crate does today, which is one zone of the tariff. For the formulas, see `docs/formulas/`.

## Status

fiskus is not on crates.io yet. Depend on it by path or git:

```toml
[dependencies]
fiskus = { path = "../fiskus" }
```

## What it computes today

For tax year 2026, `assess` returns the Einkommensteuer for taxable income in the Grundfreibetrag zone of § 32a EStG. In that zone the tax is 0 €.

- Single filing (Grundtarif): zero tax up to 12 348 €.
- Joint filing (Splittingtarif, § 32a Abs. 5 EStG): zero tax up to 24 697 € of joint taxable income.

Above these amounts, `assess` returns `Error::OutsideImplementedRange`. It does not estimate the tax. Other tariff zones will be added later.

fiskus does not compute taxable income yet. You pass in zu versteuerndes Einkommen (zvE) as an amount.

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

`FilingUnit` (Veranlagungseinheit) has two fields:

| Field | Type | Meaning |
|---|---|---|
| `filing` | `FilingStatus` | `Single` (Einzelveranlagung) or `Joint` (Zusammenveranlagung) |
| `taxable_income` | `Cents` | Zu versteuerndes Einkommen, in cents |

Amounts are `Cents`, an exact integer count of cents. Build one from euros with `Cents::from_euros(12_348)`, or from cents with `Cents::new(1_234_800)`. Fractions of a euro are rounded down before the tariff is applied, as § 32a Abs. 1 Satz 1 EStG requires. A value of `12_348.99` € is treated as `12_348` €.

## Outputs

`assess` returns an `Assessment`:

| Field | Type | Meaning |
|---|---|---|
| `taxable_income` | `Cents` | zvE rounded down to whole euros |
| `income_tax` | `Cents` | Tarifliche Einkommensteuer, whole euros |

Read the amount in euros with `income_tax.whole_euros()`, or in cents with `income_tax.get()`.

## Errors

`assess` returns `Result<Assessment, Error>`:

| Error | When |
|---|---|
| `NegativeTaxableIncome` | `taxable_income` is below zero |
| `OutsideImplementedRange { taxable_income_eur }` | Above the Grundfreibetrag zone (joint: above 24 697 € of joint income) |
| `Parameters(String)` | The bundled parameter file could not be read |

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

`TaxYear` currently has one variant, `TaxYear::Y2026`. Other years are added when their parameter files exist. Do not assume another year works.

## Not covered yet

- Income tax above the Grundfreibetrag.
- Computing zvE from income (§ 2 EStG), including Werbungskosten, Sonderausgaben and Vorsorgeaufwendungen.
- Solidaritätszuschlag and Kirchensteuer.
- Rates. There is no marginal or average rate yet.

## Disclaimer

fiskus is an independent reference engine. It is not officialjBMF or ELSTER software and it is not tax advice.
