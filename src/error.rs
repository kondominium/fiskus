// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::fmt;

/// Errors returned by [`assess`](crate::assess).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Taxable income (zu versteuerndes Einkommen) is below zero (§ 2 Abs. 5 EStG).
    NegativeTaxableIncome,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeTaxableIncome => write!(f, "taxable income is negative"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        assert_eq!(
            Error::NegativeTaxableIncome.to_string(),
            "taxable income is negative"
        );
    }

    #[test]
    fn error_debug_and_equality() {
        let e = Error::NegativeTaxableIncome;
        assert_eq!(e, Error::NegativeTaxableIncome);
        assert!(format!("{e:?}").contains("NegativeTaxableIncome"));
    }
}
