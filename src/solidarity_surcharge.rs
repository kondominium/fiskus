// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::params;
use crate::{Cents, FilingStatus, Source, TaxYear};

const FULL_RATE_NUMERATOR: i128 = 11;
const FULL_RATE_DENOMINATOR: i128 = 200;
const MITIGATION_RATE_NUMERATOR: i128 = 119;
const MITIGATION_RATE_DENOMINATOR: i128 = 1000;

/// Solidarity surcharge (Solidaritätszuschlag, § 4 SolZG) on the income tax of one filing unit.
///
/// The surcharge is 5.5 % of the income tax. In the mitigation zone it is capped at 11.9 % of the
/// amount by which the income tax exceeds the free threshold (Freigrenze) of § 3 Abs. 3 SolZG. The
/// threshold is the year's own figure: the single threshold for single filing, and the joint one
/// for joint filing. The result is in cents, rounded down.
///
/// The statute that sets the threshold is returned by [`solidarity_source`].
///
/// Returns `None` when `rules/` has no threshold for the year. fiskus does not yet archive the
/// statute for every year, so such a year has no surcharge figure rather than a guessed one.
///
/// # Examples
///
/// ```
/// use fiskus::{solidarity_surcharge, Cents, FilingStatus, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// let surcharge = solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(1_000_000));
/// assert!(surcharge.is_some());
/// ```
pub fn solidarity_surcharge(
    year: TaxYear,
    filing: FilingStatus,
    income_tax: Cents,
) -> Option<Cents> {
    let threshold_eur = solidarity_free_threshold_eur(year, filing)?;
    Some(surcharge_above(
        income_tax,
        Cents::from_euros(threshold_eur),
    ))
}

/// The free threshold (Freigrenze, § 3 Abs. 3 SolZG) of `year` for `filing`, in whole euros.
///
/// Returns `None` for a year without a threshold in `rules/`. Below the threshold no surcharge is
/// due.
///
/// # Examples
///
/// ```
/// use fiskus::{solidarity_free_threshold_eur, FilingStatus, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// let single = solidarity_free_threshold_eur(year, FilingStatus::Single).unwrap();
/// let joint = solidarity_free_threshold_eur(year, FilingStatus::Joint).unwrap();
/// assert!(joint >= single);
/// ```
pub fn solidarity_free_threshold_eur(year: TaxYear, filing: FilingStatus) -> Option<i128> {
    let solidarity = params::solidarity_for(year)?;
    Some(match filing {
        FilingStatus::Single => solidarity.free_threshold_single_eur,
        FilingStatus::Joint => solidarity.free_threshold_joint_eur,
    })
}

/// The citation, source and archived copy of the year's solidarity threshold, or `None` for a year
/// without one.
///
/// # Examples
///
/// ```
/// use fiskus::{solidarity_source, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// assert!(solidarity_source(year).is_some());
/// ```
pub fn solidarity_source(year: TaxYear) -> Option<Source> {
    params::solidarity_for(year).map(|solidarity| Source::from(&solidarity.source))
}

/// The surcharge for an income tax above a threshold, with the rates of § 4 SolZG.
fn surcharge_above(income_tax: Cents, free_threshold: Cents) -> Cents {
    let tax = income_tax.get();
    let excess = tax.saturating_sub(free_threshold.get());
    let full = tax.saturating_mul(FULL_RATE_NUMERATOR) / FULL_RATE_DENOMINATOR;
    let mitigated = excess.saturating_mul(MITIGATION_RATE_NUMERATOR) / MITIGATION_RATE_DENOMINATOR;
    Cents::new(full.min(mitigated).max(0))
}

#[cfg(kani)]
mod verification {
    use super::*;

    const BOUND_CENTS: i128 = 1_000_000_000_000_000;

    // The surcharge is never negative, never above 5.5 % of the tax, and zero at or below the threshold.
    #[kani::proof]
    fn surcharge_is_bounded_by_the_full_rate_and_zero_below_the_threshold() {
        let tax: i128 = kani::any();
        let threshold: i128 = kani::any();
        kani::assume(tax >= -BOUND_CENTS && tax <= BOUND_CENTS);
        kani::assume(threshold >= -BOUND_CENTS && threshold <= BOUND_CENTS);

        let surcharge = surcharge_above(Cents::new(tax), Cents::new(threshold)).get();
        assert!(surcharge >= 0);
        assert!(surcharge <= tax.max(0) * FULL_RATE_NUMERATOR / FULL_RATE_DENOMINATOR);
        if tax <= threshold {
            assert_eq!(surcharge, 0);
        }
    }

    // In the mitigation zone the surcharge is at most 11.9 % of the excess over the threshold.
    #[kani::proof]
    fn surcharge_is_capped_by_the_mitigation_rate() {
        let tax: i128 = kani::any();
        let threshold: i128 = kani::any();
        kani::assume(tax >= -BOUND_CENTS && tax <= BOUND_CENTS);
        kani::assume(threshold >= -BOUND_CENTS && threshold <= BOUND_CENTS);

        let surcharge = surcharge_above(Cents::new(tax), Cents::new(threshold)).get();
        let excess = (tax - threshold).max(0);
        assert!(surcharge <= excess * MITIGATION_RATE_NUMERATOR / MITIGATION_RATE_DENOMINATOR);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Sample threshold and taxes for the fixtures. They are not statute values.
    const SAMPLE_THRESHOLD_CENTS: i128 = 1_000_000;

    #[test]
    fn zero_at_and_below_the_threshold() {
        let threshold = Cents::new(SAMPLE_THRESHOLD_CENTS);
        assert_eq!(surcharge_above(threshold, threshold), Cents::new(0));
        assert_eq!(surcharge_above(Cents::new(0), threshold), Cents::new(0));
    }

    #[test]
    fn full_rate_far_above_the_threshold() {
        let tax = Cents::new(SAMPLE_THRESHOLD_CENTS * 100);
        let threshold = Cents::new(SAMPLE_THRESHOLD_CENTS);
        assert_eq!(
            surcharge_above(tax, threshold),
            Cents::new(tax.get() * 11 / 200)
        );
    }

    #[test]
    fn mitigation_zone_caps_the_surcharge_just_above_the_threshold() {
        let threshold = Cents::new(SAMPLE_THRESHOLD_CENTS);
        let just_above = Cents::new(SAMPLE_THRESHOLD_CENTS + 100);
        assert_eq!(surcharge_above(just_above, threshold), Cents::new(11));
    }

    #[test]
    fn overflow_saturates_instead_of_panicking() {
        assert!(surcharge_above(Cents::new(i128::MAX), Cents::new(0)).get() >= 0);
    }

    #[test]
    fn years_without_a_threshold_have_no_surcharge() {
        for year in TaxYear::ALL {
            let has_threshold = params::solidarity_for(year).is_some();
            let surcharge =
                solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(1_000_000));
            assert_eq!(surcharge.is_some(), has_threshold, "{year:?}");
        }
    }

    #[test]
    fn every_year_with_a_threshold_is_zero_at_it_for_both_filings() {
        for year in TaxYear::ALL {
            let Some(solidarity) = params::solidarity_for(year) else {
                continue;
            };
            let single = Cents::from_euros(solidarity.free_threshold_single_eur);
            let joint = Cents::from_euros(solidarity.free_threshold_joint_eur);
            assert_eq!(
                solidarity_surcharge(year, FilingStatus::Single, single),
                Some(Cents::new(0)),
                "{year:?}"
            );
            assert_eq!(
                solidarity_surcharge(year, FilingStatus::Joint, joint),
                Some(Cents::new(0)),
                "{year:?}"
            );
        }
    }

    #[test]
    fn joint_filers_use_the_joint_threshold() {
        for year in TaxYear::ALL {
            let Some(solidarity) = params::solidarity_for(year) else {
                continue;
            };
            let tax = Cents::from_euros(solidarity.free_threshold_single_eur);
            assert_eq!(
                solidarity_surcharge(year, FilingStatus::Joint, tax),
                Some(Cents::new(0)),
                "{year:?}"
            );
        }
    }

    proptest! {
        #[test]
        fn surcharge_never_decreases_as_tax_rises(a in 0..i128::MAX / 1000, b in 0..i128::MAX / 1000, threshold in 0..1_000_000_000i128) {
            let (low, high) = (a.min(b), a.max(b));
            let threshold = Cents::new(threshold);
            prop_assert!(surcharge_above(Cents::new(low), threshold) <= surcharge_above(Cents::new(high), threshold));
        }

        #[test]
        fn surcharge_is_never_negative(tax in any::<i128>(), threshold in any::<i128>()) {
            prop_assert!(surcharge_above(Cents::new(tax), Cents::new(threshold)).get() >= 0);
        }

        #[test]
        fn surcharge_of_a_listed_year_is_never_negative(index in 0..TaxYear::ALL.len(), tax in any::<i128>(), joint in any::<bool>()) {
            let filing = if joint { FilingStatus::Joint } else { FilingStatus::Single };
            if let Some(surcharge) = solidarity_surcharge(TaxYear::ALL[index], filing, Cents::new(tax)) {
                prop_assert!(surcharge.get() >= 0);
            }
        }
    }
}
