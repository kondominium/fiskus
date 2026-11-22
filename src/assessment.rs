// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::Cents;

/// Result of assessing one filing unit for a tax year.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assessment {
    /// Taxable income (zu versteuerndes Einkommen), rounded down to whole euros, in cents.
    pub taxable_income: Cents,
    /// Income tax (tarifliche Einkommensteuer), in cents. Always a whole number of euros.
    pub income_tax: Cents,
}
