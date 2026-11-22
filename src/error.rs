// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::fmt;

/// Errors returned by [`assess`](crate::assess).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Taxable income (zu versteuerndes Einkommen) is below zero (§ 2 Abs. 5 EStG).
    NegativeTaxableIncome,
    /// Taxable income is above the basic allowance zone (Grundfreibetrag). The tariff zones beyond it are not implemented yet.
    OutsideImplementedRange {
        /// Taxable income in whole euros.
        taxable_income_eur: i128,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeTaxableIncome => write!(f, "taxable income is negative"),
            Self::OutsideImplementedRange { taxable_income_eur } => write!(
                f,
                "taxable income {taxable_income_eur} EUR is above the implemented tariff range"
            ),
        }
    }
}

impl std::error::Error for Error {}
