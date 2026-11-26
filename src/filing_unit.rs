// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::Cents;

/// Filing status (Veranlagungsform, § 26 EStG).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilingStatus {
    /// One person taxed alone, on the basic tariff (Grundtarif, § 32a Abs. 1 EStG).
    Single,
    /// Joint assessment (Zusammenveranlagung, § 26b EStG), splitting tariff (Splittingtarif).
    Joint,
}

/// One filing unit (Veranlagungseinheit): one spouse, or two spouses filing jointly.
///
/// Taxable income is given directly until the § 2 EStG computation exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilingUnit {
    /// Single or joint filing.
    pub filing: FilingStatus,
    /// Taxable income (zu versteuerndes Einkommen), in cents.
    pub taxable_income: Cents,
}
