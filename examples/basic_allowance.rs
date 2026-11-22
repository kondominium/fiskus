// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Assesses income tax with the basic allowance (Grundfreibetrag) and the tariff zones, for VZ 2026 and an earlier year.
//!
//! Run with `cargo run --example basic_allowance`.

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur};

fn main() -> Result<(), Error> {
    println!(
        "basic allowance for VZ 2026: {} EUR",
        basic_allowance_eur(TaxYear::Y2026)
    );

    let single_at_allowance = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(12_348),
    };
    let assessment = assess(TaxYear::Y2026, &single_at_allowance)?;
    println!(
        "single, 12348 EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let single_mid_range = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(30_000),
    };
    let assessment = assess(TaxYear::Y2026, &single_mid_range)?;
    println!(
        "single, 30000 EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let joint = FilingUnit {
        filing: FilingStatus::Joint,
        taxable_income: Cents::from_euros(100_000),
    };
    let assessment = assess(TaxYear::Y2026, &joint)?;
    println!(
        "joint, 100000 EUR: income tax {} cents",
        assessment.income_tax.get()
    );

    let assessment = assess(TaxYear::Y2009, &single_mid_range)?;
    println!(
        "single, 30000 EUR, VZ 2009: income tax {} cents",
        assessment.income_tax.get()
    );

    Ok(())
}
