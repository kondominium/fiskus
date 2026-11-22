// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Explains the income tax for one income: the zone, the unrounded tax and the source of the tariff.
//!
//! Run with `cargo run --example explain_tax`.

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, explain};

fn main() -> Result<(), Error> {
    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(30_000),
    };
    let explanation = explain(TaxYear::Y2026, &unit)?;
    if let Some(zone) = explanation.zone {
        println!("zone {} from {} EUR", zone.number, zone.from_eur);
    }
    println!(
        "unrounded tax {:.2} EUR, income tax {} EUR",
        explanation.unrounded_basis_tax.numerator as f64
            / explanation.unrounded_basis_tax.denominator as f64,
        explanation.income_tax_eur
    );
    println!(
        "average rate {:.1} %, marginal rate {:.1} %",
        explanation.average_rate * 100.0,
        explanation.marginal_rate * 100.0
    );
    println!("source: {}", explanation.source);
    println!("reference: {}", explanation.reference);
    Ok(())
}
