// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Assesses income tax at the basic allowance (Grundfreibetrag) for VZ 2026.
//!
//! Run with `cargo run --example basic_allowance`.

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur};

fn main() -> Result<(), Error> {
    let year = TaxYear::from_year(2026).expect("2026 is a supported year");
    let allowance = basic_allowance_eur(year);

    let single = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(allowance),
    };
    let assessment = assess(year, &single)?;
    println!(
        "single, {allowance} EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let joint_limit = allowance * 2 + 1;
    let joint = FilingUnit {
        filing: FilingStatus::Joint,
        taxable_income: Cents::from_euros(joint_limit),
    };
    let assessment = assess(year, &joint)?;
    println!(
        "joint, {joint_limit} EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let above = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(allowance + 1),
    };
    match assess(year, &above) {
        Ok(assessment) => println!("unexpected: {assessment:?}"),
        Err(error) => println!("single, {} EUR: {error}", allowance + 1),
    }

    Ok(())
}
