// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Assesses income tax for the latest listed year, for both filings, at sample incomes.
//!
//! The incomes are multiples of the basic allowance, so the example stays correct when a new year
//! is added. Run with `cargo run --example income_tax`.

#![expect(
    clippy::print_stdout,
    clippy::expect_used,
    reason = "an example prints its result, and stops with a message if the year list is empty"
)]

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur};

fn main() -> Result<(), Error> {
    let year = *TaxYear::ALL.last().expect("at least one year is listed");
    let allowance = basic_allowance_eur(year);
    println!("VZ {}: basic allowance {allowance} EUR", year.year());

    for multiple in [1, 2, 5] {
        let euros = allowance * multiple;
        for filing in [FilingStatus::Single, FilingStatus::Joint] {
            let unit = FilingUnit {
                filing,
                taxable_income: Cents::from_euros(euros),
            };
            let assessment = assess(year, &unit)?;
            println!(
                "{filing:?}, taxable income {euros} EUR: income tax {} EUR",
                assessment.income_tax.whole_euros()
            );
        }
    }

    Ok(())
}
