// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::params;
use crate::{Assessment, Cents, Error, FilingStatus, FilingUnit, TaxYear};

/// Tarifliche Einkommensteuer (§ 32a EStG). Implements only the basic allowance zone so far.
pub fn assess(year: TaxYear, unit: &FilingUnit) -> Result<Assessment, Error> {
    let parameters = params::for_year(year)?;
    basic_allowance_zone(unit, parameters.basic_allowance.value_eur)
}

fn basic_allowance_zone(unit: &FilingUnit, basic_allowance_eur: i128) -> Result<Assessment, Error> {
    if unit.taxable_income.get() < 0 {
        return Err(Error::NegativeTaxableIncome);
    }

    let taxable_income_eur = unit.taxable_income.whole_euros();
    let basis_eur = match unit.filing {
        FilingStatus::Single => taxable_income_eur,
        FilingStatus::Joint => taxable_income_eur / 2,
    };

    if basis_eur > basic_allowance_eur {
        return Err(Error::OutsideImplementedRange { taxable_income_eur });
    }

    Ok(Assessment {
        taxable_income: Cents::from_euros(taxable_income_eur),
        income_tax: Cents::new(0),
    })
}

#[cfg(kani)]
mod verification {
    use super::*;

    // 10^12 cents is 10 billion euros, far above any real taxable income. Kani
    // with unbounded i128 division and multiplication does not finish in CI time,
    // so the boundary proof is limited to this range. The no-panic harness covers
    // the full i128 range.
    const BOUND_CENTS: i128 = 1_000_000_000_000;

    #[kani::proof]
    fn basic_allowance_zone_boundary_is_exact() {
        let cents: i128 = kani::any();
        let basic_allowance_eur: i128 = kani::any();
        kani::assume(cents >= -BOUND_CENTS && cents <= BOUND_CENTS);
        kani::assume(basic_allowance_eur >= 0 && basic_allowance_eur <= BOUND_CENTS);
        let joint: bool = kani::any();
        let filing = if joint {
            FilingStatus::Joint
        } else {
            FilingStatus::Single
        };
        let unit = FilingUnit {
            filing,
            taxable_income: Cents::new(cents),
        };

        let result = basic_allowance_zone(&unit, basic_allowance_eur);

        if cents < 0 {
            assert_eq!(result, Err(Error::NegativeTaxableIncome));
            return;
        }

        let whole_euros = cents.div_euclid(100);
        let basis_eur = match filing {
            FilingStatus::Single => whole_euros,
            FilingStatus::Joint => whole_euros / 2,
        };

        if basis_eur <= basic_allowance_eur {
            let assessment = result.unwrap();
            assert_eq!(assessment.income_tax, Cents::new(0));
            assert_eq!(assessment.taxable_income, Cents::from_euros(whole_euros));
        } else {
            assert!(matches!(result, Err(Error::OutsideImplementedRange { .. })));
        }
    }

    #[kani::proof]
    fn basic_allowance_zone_never_panics() {
        let cents: i128 = kani::any();
        let basic_allowance_eur: i128 = kani::any();
        let joint: bool = kani::any();
        let filing = if joint {
            FilingStatus::Joint
        } else {
            FilingStatus::Single
        };
        let unit = FilingUnit {
            filing,
            taxable_income: Cents::new(cents),
        };

        let _ = basic_allowance_zone(&unit, basic_allowance_eur);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Grundfreibetrag for VZ 2026 (§ 32a Abs. 1 Satz 2 Nr. 1 EStG).
    const BASIC_ALLOWANCE_EUR: i128 = 12_348;
    // Largest joint zvE still in the zone: the half (12 348.50) rounds down to 12 348.
    const JOINT_ZONE_EUR: i128 = 24_697;

    fn single(cents: i128) -> FilingUnit {
        FilingUnit {
            filing: FilingStatus::Single,
            taxable_income: Cents::new(cents),
        }
    }

    fn joint(cents: i128) -> FilingUnit {
        FilingUnit {
            filing: FilingStatus::Joint,
            taxable_income: Cents::new(cents),
        }
    }

    // Single filer exactly at the allowance: tax is 0 and zvE is kept as given.
    #[test]
    fn single_is_tax_free_up_to_basic_allowance() {
        let result = assess(TaxYear::Y2026, &single(BASIC_ALLOWANCE_EUR * 100)).unwrap();
        assert_eq!(result.income_tax, Cents::new(0));
        assert_eq!(
            result.taxable_income,
            Cents::from_euros(BASIC_ALLOWANCE_EUR)
        );
    }

    // 12 348.99 € is treated as 12 348 €, so it is still in the zone (§ 32a Abs. 1 Satz 1).
    #[test]
    fn single_cents_are_rounded_down_before_the_zone_check() {
        let result = assess(TaxYear::Y2026, &single(BASIC_ALLOWANCE_EUR * 100 + 99)).unwrap();
        assert_eq!(
            result.taxable_income,
            Cents::from_euros(BASIC_ALLOWANCE_EUR)
        );
        assert_eq!(result.income_tax, Cents::new(0));
    }

    // 12 349 € is the first amount outside the zone, so the tariff zones are not implemented for it.
    #[test]
    fn single_one_euro_above_basic_allowance_is_outside_the_implemented_range() {
        let error = assess(TaxYear::Y2026, &single((BASIC_ALLOWANCE_EUR + 1) * 100)).unwrap_err();
        assert_eq!(
            error,
            Error::OutsideImplementedRange {
                taxable_income_eur: BASIC_ALLOWANCE_EUR + 1
            }
        );
    }

    // Joint filer at 24 697 €: the Splitting half is 12 348 €, so tax is 0 (§ 32a Abs. 5 EStG).
    #[test]
    fn joint_is_tax_free_up_to_the_split_zone_boundary() {
        let result = assess(TaxYear::Y2026, &joint(JOINT_ZONE_EUR * 100)).unwrap();
        assert_eq!(result.income_tax, Cents::new(0));
    }

    // Joint filer at 24 698 €: the half is 12 349 €, just outside the zone.
    #[test]
    fn joint_one_euro_above_the_split_zone_is_outside_the_implemented_range() {
        let error = assess(TaxYear::Y2026, &joint((JOINT_ZONE_EUR + 1) * 100)).unwrap_err();
        assert_eq!(
            error,
            Error::OutsideImplementedRange {
                taxable_income_eur: JOINT_ZONE_EUR + 1
            }
        );
    }

    // zvE below zero is an input error, not a tax of 0 (§ 2 Abs. 5 EStG).
    #[test]
    fn negative_taxable_income_is_rejected() {
        assert_eq!(
            assess(TaxYear::Y2026, &single(-1)).unwrap_err(),
            Error::NegativeTaxableIncome
        );
    }

    // Random inputs: the checks above, over the whole range, with any cents part.
    proptest! {
        #[test]
        fn single_zone_is_tax_free_for_any_cents(euros in 0..=BASIC_ALLOWANCE_EUR, cents in 0..100i128) {
            let result = assess(TaxYear::Y2026, &single(euros * 100 + cents)).unwrap();
            prop_assert_eq!(result.income_tax, Cents::new(0));
            prop_assert_eq!(result.taxable_income, Cents::from_euros(euros));
        }

        #[test]
        fn joint_zone_is_tax_free_for_any_cents(euros in 0..=JOINT_ZONE_EUR, cents in 0..100i128) {
            let result = assess(TaxYear::Y2026, &joint(euros * 100 + cents)).unwrap();
            prop_assert_eq!(result.income_tax, Cents::new(0));
        }

        // Any i128 cents value, single or joint, returns Ok or Err but never panics.
        #[test]
        fn assess_never_panics(cents in any::<i128>(), joint_filing in any::<bool>()) {
            let filing = if joint_filing { FilingStatus::Joint } else { FilingStatus::Single };
            let unit = FilingUnit { filing, taxable_income: Cents::new(cents) };
            let _ = assess(TaxYear::Y2026, &unit);
        }
    }
}
