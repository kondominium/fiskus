// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::Cents;

/// Veranlagungsform (§ 26 EStG).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilingStatus {
    /// Einzelveranlagung (§ 26a EStG), Grundtarif.
    Single,
    /// Zusammenveranlagung (§ 26b EStG), Splittingtarif.
    Joint,
}

/// Veranlagungseinheit: the one or two spouses assessed together. Taxable income
/// (zu versteuerndes Einkommen) is given directly until the § 2 EStG computation exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilingUnit {
    pub filing: FilingStatus,
    pub taxable_income: Cents,
}
