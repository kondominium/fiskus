// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Assesses income tax at the basic allowance (Grundfreibetrag) for VZ 2026.
//!
//! Run with `cargo run --example basic_allowance`.

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess};

fn main() -> Result<(), Error> {
    let single = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(12_348),
    };
    let assessment = assess(TaxYear::Y2026, &single)?;
    println!(
        "single, 12348 EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let joint = FilingUnit {
        filing: FilingStatus::Joint,
        taxable_income: Cents::from_euros(24_697),
    };
    let assessment = assess(TaxYear::Y2026, &joint)?;
    println!(
        "joint, 24697 EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let above = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(12_349),
    };
    match assess(TaxYear::Y2026, &above) {
        Ok(assessment) => println!("unexpected: {assessment:?}"),
        Err(error) => println!("single, 12349 EUR: {error}"),
    }

    Ok(())
}
