//! # fiskus
//!
//! Independent reference engine for German income tax (Einkommensteuer).
//!
//! **Not official BMF or ELSTER software. Not tax advice.**
//!
//! Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//!
//! So far only the Grundfreibetrag zone of § 32a EStG is implemented, for VZ 2003 to 2026.

mod assessment;
mod error;
mod filing_unit;
mod money;
mod params;
mod tariff;
mod year;

pub use assessment::Assessment;
pub use error::Error;
pub use filing_unit::{FilingStatus, FilingUnit};
pub use money::Cents;
pub use tariff::{assess, basic_allowance_eur};
pub use year::TaxYear;
