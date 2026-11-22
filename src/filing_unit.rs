// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::Cents;

/// Filing status (Veranlagungsform, § 26 EStG).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilingStatus {
    /// Single assessment (Einzelveranlagung, § 26a EStG), basic tariff (Grundtarif).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filing_status_variants_are_distinct() {
        assert_ne!(FilingStatus::Single, FilingStatus::Joint);
        assert_eq!(FilingStatus::Single, FilingStatus::Single);
    }

    #[test]
    fn filing_unit_is_copy_and_equal() {
        let u = FilingUnit {
            filing: FilingStatus::Joint,
            taxable_income: Cents::from_euros(100),
        };
        let v = u;
        assert_eq!(u, v);
        assert_eq!(v.filing, FilingStatus::Joint);
        assert_eq!(v.taxable_income, Cents::from_euros(100));
    }

    #[test]
    fn filing_unit_stores_cents_exactly() {
        let u = FilingUnit {
            filing: FilingStatus::Single,
            taxable_income: Cents::new(12_345),
        };
        assert_eq!(u.taxable_income.get(), 12_345);
    }
}
