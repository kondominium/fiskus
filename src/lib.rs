//! # fiskus
//!
//! Independent reference engine for German income tax (Einkommensteuer).
//!
//! **Not official BMF or ELSTER software. Not tax advice.**
//!
//! Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//!
//! fiskus computes the income tax of § 32a EStG for every year in [`TaxYear::ALL`] that has tariff zones,
//! and the basic allowance for every listed year. See the guide in `docs/guides` for how the calculation works.

mod assessment;
mod error;
mod filing_unit;
mod money;
mod params;
mod solidarity_surcharge;
mod tariff;
mod year;

pub use assessment::Assessment;
pub use error::Error;
pub use filing_unit::{FilingStatus, FilingUnit};
pub use money::Cents;
pub use solidarity_surcharge::{
    solidarity_free_threshold_eur, solidarity_source, solidarity_surcharge,
};
pub use tariff::{
    Explanation, Fraction, Source, ZoneStep, assess, basic_allowance_eur, explain, tariff_zones,
};
pub use year::TaxYear;
