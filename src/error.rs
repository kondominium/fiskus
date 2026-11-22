// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Zu versteuerndes Einkommen cannot be negative (§ 2 Abs. 5 EStG).
    NegativeTaxableIncome,
    /// Above the Grundfreibetrag, the tariff zones are not implemented yet.
    OutsideImplementedRange { taxable_income_eur: i128 },
    /// The bundled parameter file for the year could not be read.
    Parameters(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeTaxableIncome => write!(f, "taxable income is negative"),
            Self::OutsideImplementedRange { taxable_income_eur } => write!(
                f,
                "taxable income {taxable_income_eur} EUR is above the implemented tariff range"
            ),
            Self::Parameters(reason) => write!(f, "invalid tax parameters: {reason}"),
        }
    }
}

impl std::error::Error for Error {}
