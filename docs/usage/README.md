# Using fiskus

This page is for developers and agents who call fiskus from Rust. It describes what the crate does today, and how to call it. For the reasoning behind the figures, read the [guide](../guide/README.md). For the formulas, read [Formulas](../formulas/grundfreibetrag.md).

## Status

fiskus is not on crates.io yet. Depend on it by path or git:

```toml
[dependencies]
fiskus = { path = "../fiskus" }
```

## What it computes

- `assess` returns the income tax (Einkommensteuer) for taxable income (zu versteuerndes Einkommen, zvE) under the tariff of § 32a EStG, for single and joint filing.
- `explain` returns the same tax with its steps: the zone, the unrounded tax, the average and marginal rates, and the citation of the year's tariff.
- `tariff_zones` returns the zones of a year. `basic_allowance_eur` returns its basic allowance.
- `solidarity_surcharge` returns the solidarity surcharge (Solidaritätszuschlag) for a year, a filing status and an income tax, with the year's free threshold (Freigrenze). It returns `None` for a year without a threshold. `solidarity_free_threshold_eur` and `solidarity_source` give the threshold and its citation.

Every year fiskus lists is in `TaxYear::ALL`. A year without tariff zones returns its basic allowance as tax-free, and reports an income above it as outside the implemented range.

fiskus does not compute taxable income yet. You pass it in as an amount.

## Choose a year

`TaxYear` has one variant per listed year. Pick one from the list, or look it up by calendar year:

```rust
use fiskus::TaxYear;

let latest = *TaxYear::ALL.last().expect("at least one year is listed");
let by_number = TaxYear::from_year(latest.year()); // Some(latest)
let unknown = TaxYear::from_year(0);              // None
```

## Quick start

The example below uses the basic allowance of the latest year, so it stays correct when a year is added. It returns zero tax, because the zone of zero tax runs up to the allowance.

```rust
use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur};

fn main() -> Result<(), Error> {
    let year = *TaxYear::ALL.last().expect("at least one year is listed");
    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(basic_allowance_eur(year)),
    };

    let assessment = assess(year, &unit)?;
    assert_eq!(assessment.income_tax, Cents::new(0));
    Ok(())
}
```

Run the bundled examples with `cargo run --example income_tax` and `cargo run --example explain_tax`, or with `make run-example-income-tax` and `make run-example-explain-tax`.

## Inputs

The filing unit (Veranlagungseinheit) is `FilingUnit`. It has two fields:

| Field | Type | Meaning |
|---|---|---|
| `filing` | `FilingStatus` | `Single` (single assessment, Einzelveranlagung) or `Joint` (joint assessment, Zusammenveranlagung) |
| `taxable_income` | `Cents` | Taxable income (zu versteuerndes Einkommen), in cents |

Build an amount from euros with `Cents::from_euros(12_348)`, or from cents with `Cents::new(1_234_800)`. Fractions of a euro are rounded down before the tariff applies, as § 32a Abs. 1 Satz 1 EStG requires. An amount of 1 000.99 € is taken as 1 000 €.

## Outputs

`assess` returns an `Assessment`:

| Field | Type | Meaning |
|---|---|---|
| `taxable_income` | `Cents` | Taxable income, rounded down to whole euros |
| `income_tax` | `Cents` | Tariff tax (tarifliche Einkommensteuer), whole euros |

`explain` returns an `Explanation`. Its fields are the zone (`zone`, a `ZoneStep` or `None` in the zero zone), `unrounded_basis_tax` (a `Fraction`), `income_tax_eur`, `average_rate`, `marginal_rate`, and `source` (a `Source` with the citation, the source and the archived copy, or `None` for a year without zones).

## Errors

`assess` and `explain` return `Result<_, Error>`:

| Error | When |
|---|---|
| `NegativeTaxableIncome` | `taxable_income` is below zero. An input error, never a zero tax |
| `OutsideImplementedRange { taxable_income_eur }` | The income is above the basic allowance, and fiskus cannot compute its tax: the year has no tariff zones, or the amount is too large for `i128` |

`Error` implements `std::error::Error` and `Display`. Match on the variant, not on the message.

```rust
use fiskus::{assess, Cents, Error, FilingStatus, FilingUnit, TaxYear};

let year = *TaxYear::ALL.last().unwrap();
let unit = FilingUnit {
    filing: FilingStatus::Single,
    taxable_income: Cents::new(-1),
};
match assess(year, &unit) {
    Ok(assessment) => println!("tax: {}", assessment.income_tax.whole_euros()),
    Err(Error::NegativeTaxableIncome) => println!("fix the input"),
    Err(error) => println!("cannot compute: {error}"),
}
```

## Explanations and rates

```rust
use fiskus::{Cents, FilingStatus, FilingUnit, TaxYear, explain};

let year = *TaxYear::ALL.last().unwrap();
let unit = FilingUnit {
    filing: FilingStatus::Single,
    taxable_income: Cents::from_euros(1_000_000),
};
let explanation = explain(year, &unit).unwrap();
println!("zone {:?}, marginal rate {:.1} %", explanation.zone.map(|z| z.number), explanation.marginal_rate * 100.0);
```

## Solidarity surcharge

```rust
use fiskus::{Cents, FilingStatus, TaxYear, solidarity_surcharge};

let year = *TaxYear::ALL.last().expect("at least one year is listed");
let surcharge = solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(1_000_000));
println!("{:?}", surcharge.map(|cents| cents.whole_euros()));
```

The threshold of the year comes from its rule file. For a year without one, the result is `None`. See the [rates and surcharge](../guide/06-rates-and-surcharge.md) page.

## What is not in fiskus yet

- Taxable income from income (§ 2 EStG), allowances, special expenses and child allowances.
- The special cases of § 32a Abs. 1 Satz 2 EStG and the tariff for a surviving spouse.
- Church tax (Kirchensteuer).
- Solidarity surcharge thresholds for the years whose statute is not yet archived.

## Stability

Before the crate is published, the API may change. The variants of `TaxYear` grow with each year. Match on them with a fallback arm, so a new year does not break your code.
