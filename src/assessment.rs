// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::Cents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assessment {
    /// Zu versteuerndes Einkommen, rounded down to the whole euro (§ 32a Abs. 1 Satz 1 EStG).
    pub taxable_income: Cents,
    /// Tarifliche Einkommensteuer, in whole euros.
    pub income_tax: Cents,
}
