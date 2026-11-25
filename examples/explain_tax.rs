// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Explains the tax steps for the latest listed year: the zones, one explanation, and the surcharge.
//!
//! Run with `cargo run --example explain_tax`.

#![expect(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    reason = "an example prints the steps it computes, in decimal for a reader; it stops with a message if the year list is empty"
)]

use fiskus::{
    Cents, Error, FilingStatus, FilingUnit, TaxYear, explain, solidarity_free_threshold_eur,
    solidarity_surcharge, tariff_zones,
};

fn main() -> Result<(), Error> {
    let year = *TaxYear::ALL.last().expect("at least one year is listed");

    println!("VZ {}: zones of the tariff", year.year());
    for zone in tariff_zones(year) {
        println!(
            "  zone {}: from {} EUR, base {} EUR, step {} EUR",
            zone.number, zone.from_eur, zone.base_eur, zone.step_eur
        );
    }

    let unit = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::from_euros(
            tariff_zones(year).last().map_or(0, |zone| zone.from_eur),
        ),
    };
    let explanation = explain(year, &unit)?;
    println!(
        "single, taxable income {} EUR: zone {:?}, unrounded tax {:.2} EUR, income tax {} EUR",
        explanation.taxable_income_eur,
        explanation.zone.map(|zone| zone.number),
        explanation.unrounded_basis_tax.numerator as f64
            / explanation.unrounded_basis_tax.denominator as f64,
        explanation.income_tax_eur,
    );
    println!(
        "average rate {:.2} %, marginal rate {:.2} %",
        explanation.average_rate * 100.0,
        explanation.marginal_rate * 100.0,
    );
    if let Some(source) = explanation.source {
        println!("source: {} ({})", source.citation, source.reference);
    }

    match solidarity_free_threshold_eur(year, unit.filing) {
        Some(threshold) => {
            let surcharge = solidarity_surcharge(
                year,
                unit.filing,
                Cents::from_euros(explanation.income_tax_eur),
            );
            println!(
                "solidarity surcharge: free threshold {threshold} EUR, surcharge {} EUR",
                surcharge.map_or(0, Cents::whole_euros)
            );
        }
        None => println!("solidarity surcharge: no free threshold listed for this year"),
    }

    Ok(())
}
