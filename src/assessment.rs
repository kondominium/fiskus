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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cents;

    #[test]
    fn assessment_equality_and_debug() {
        let a = Assessment {
            taxable_income: Cents::from_euros(100),
            income_tax: Cents::from_euros(10),
        };
        let b = a.clone();
        assert_eq!(a, b);
        assert!(format!("{a:?}").contains("Assessment"));
    }

    #[test]
    fn assessment_fields_store_cents() {
        let a = Assessment {
            taxable_income: Cents::new(123),
            income_tax: Cents::new(456),
        };
        assert_eq!(a.taxable_income.get(), 123);
        assert_eq!(a.income_tax.get(), 456);
    }
}
