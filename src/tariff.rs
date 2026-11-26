// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::params::{self, Zone};
use crate::{Assessment, Cents, Error, FilingStatus, FilingUnit, TaxYear};

/// Assesses income tax for one filing unit in `year`.
///
/// Taxable income is given in cents and rounded down to whole euros before the tariff applies.
/// The tax is always a whole number of euros, in cents. Joint filers are taxed on half of their
/// income and the result is doubled (§ 32a Abs. 5 EStG).
///
/// # Errors
///
/// - [`Error::NegativeTaxableIncome`] for negative income.
/// - [`Error::OutsideImplementedRange`] above the basic allowance when fiskus has no tariff zones
///   for the year, or when the amount is too large to compute in `i128`.
///
/// # Examples
///
/// ```
/// use fiskus::{assess, basic_allowance_eur, Cents, FilingStatus, FilingUnit, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// let unit = FilingUnit {
///     filing: FilingStatus::Single,
///     taxable_income: Cents::from_euros(basic_allowance_eur(year)),
/// };
/// let assessment = assess(year, &unit).unwrap();
/// assert_eq!(assessment.income_tax, Cents::new(0));
/// ```
pub fn assess(year: TaxYear, unit: &FilingUnit) -> Result<Assessment, Error> {
    explain(year, unit).map(|explanation| Assessment {
        taxable_income: Cents::from_euros(explanation.taxable_income_eur),
        income_tax: Cents::from_euros(explanation.income_tax_eur),
    })
}

/// Explains how the income tax for one filing unit in `year` is computed.
///
/// The result has the same tax as [`assess`], plus the zone used, the unrounded tax, the rates,
/// and the citation of the year's tariff. Amounts are whole euros. `unrounded_basis_tax` is a
/// fraction of euros, before the tax is rounded down.
///
/// # Errors
///
/// Returns the same errors as [`assess`].
///
/// # Examples
///
/// ```
/// use fiskus::{assess, explain, Cents, FilingStatus, FilingUnit, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// let unit = FilingUnit {
///     filing: FilingStatus::Single,
///     taxable_income: Cents::from_euros(1_000_000),
/// };
/// let explanation = explain(year, &unit).unwrap();
/// assert_eq!(explanation.income_tax_eur, assess(year, &unit).unwrap().income_tax.whole_euros());
/// assert!(explanation.marginal_rate < 1.0);
/// ```
pub fn explain(year: TaxYear, unit: &FilingUnit) -> Result<Explanation, Error> {
    let taxable_income_eur = taxable_euros(unit.taxable_income)?;
    let outside = Error::OutsideImplementedRange { taxable_income_eur };
    let allowance_eur = basic_allowance_eur(year);
    let zones = params::tariff_for(year);
    let basis_eur = filing_basis_eur(unit.filing, taxable_income_eur);
    let table = params::table_step_for(year);
    let basis = basis_tax(zones, allowance_eur, table, basis_eur).ok_or_else(|| outside.clone())?;
    let income_tax_eur = income_tax_eur(unit.filing, basis.tax_eur).ok_or(outside)?;

    Ok(Explanation {
        year,
        filing: unit.filing,
        taxable_income_eur,
        basis_eur,
        tariff_basis_eur: basis.tariff_basis_eur,
        basic_allowance_eur: allowance_eur,
        zone: basis
            .zone
            .and_then(|index| zones.get(index).map(|zone| zone_step(index, zone))),
        unrounded_basis_tax: basis.unrounded,
        basis_tax_eur: basis.tax_eur,
        income_tax_eur,
        average_rate: ratio(income_tax_eur, taxable_income_eur),
        marginal_rate: basis
            .zone
            .and_then(|index| zones.get(index))
            .map_or(0.0, |zone| marginal_rate(zone, basis.tariff_basis_eur)),
        source: params::tariff_source_for(year).map(Source::from),
    })
}

/// The progressive zones of the tariff of `year`, from zone 2 upward, with their coefficients.
///
/// Zone 1 is the basic allowance (zero tax). It is returned by [`basic_allowance_eur`]. The list
/// is empty for a year whose tariff fiskus has not yet implemented. Amounts are whole euros, and
/// coefficients are in hundredths of a euro, see [`ZoneStep`].
///
/// # Examples
///
/// ```
/// use fiskus::{basic_allowance_eur, tariff_zones, TaxYear};
///
/// let year = *TaxYear::ALL.last().unwrap();
/// let zones = tariff_zones(year);
/// assert_eq!(zones[0].number, 2);
/// assert_eq!(zones[0].from_eur, basic_allowance_eur(year) + 1);
/// ```
#[must_use]
pub fn tariff_zones(year: TaxYear) -> Vec<ZoneStep> {
    params::tariff_for(year)
        .iter()
        .enumerate()
        .map(|(index, zone)| zone_step(index, zone))
        .collect()
}

/// Basic allowance (Grundfreibetrag, § 32a Abs. 1 Satz 2 Nr. 1 EStG) for `year`, in whole euros.
///
/// The amount is the zone of zero tax for single filers. It is not the last euro that pays
/// no tax: tax is rounded down to whole euros, so a few euros above it can still pay 0 €.
///
/// # Examples
///
/// ```
/// use fiskus::{basic_allowance_eur, TaxYear};
///
/// let year = *TaxYear::ALL.first().unwrap();
/// assert!(basic_allowance_eur(year) > 0);
/// ```
#[must_use]
pub fn basic_allowance_eur(year: TaxYear) -> i128 {
    params::for_year(year).basic_allowance.value_eur
}

/// Income tax with its steps: the zone, the unrounded tax, the rates and the citation.
///
/// Returned by [`explain`]. Amounts are whole euros. `unrounded_basis_tax` is the tax on the
/// basis before rounding down, and `income_tax_eur` is the tax for the filing unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Explanation {
    /// Assessment year.
    pub year: TaxYear,
    /// Filing status the tax was computed for.
    pub filing: FilingStatus,
    /// Taxable income (zvE) in whole euros, rounded down from cents.
    pub taxable_income_eur: i128,
    /// Basis for the tariff: the taxable income for single filers, half of it for joint filers.
    pub basis_eur: i128,
    /// Basis the zone formula uses: `basis_eur` after the § 32a Abs. 2 table step. Equal to `basis_eur`
    /// for a year without a table step.
    pub tariff_basis_eur: i128,
    /// Basic allowance (Grundfreibetrag) of the year, in whole euros.
    pub basic_allowance_eur: i128,
    /// The zone that applies to the basis, or `None` in the zero zone.
    pub zone: Option<ZoneStep>,
    /// Tax on the basis before rounding down, in euros.
    pub unrounded_basis_tax: Fraction,
    /// Tax on the basis, rounded down to whole euros.
    pub basis_tax_eur: i128,
    /// Income tax of the filing unit, in whole euros.
    pub income_tax_eur: i128,
    /// Average tax rate: income tax divided by taxable income, `0.0` for zero income.
    pub average_rate: f64,
    /// Marginal tax rate: the tax on the next euro of taxable income, from the zone's derivative.
    pub marginal_rate: f64,
    /// Where the year's tariff comes from. `None` for a year without tariff zones.
    pub source: Option<Source>,
}

/// One progressive zone of the tariff.
///
/// The tax on a basis `x` inside the zone is `a·t² + b·t + c`, with `t = (x − base_eur) / step_eur`.
/// The coefficients `a`, `b` and `c` are `quadratic_h`, `linear_h` and `constant_h` in hundredths
/// of a euro, so `quadratic_h = 91451` means `a = 914.51`. Linear zones have `quadratic_h = 0`,
/// and a `step_eur` of 1, or of 10 where the slope has a half-hundredth (0,485 is `linear_h = 485`
/// with `step_eur = 10`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneStep {
    /// Zone number in § 32a Abs. 1 EStG: 2 for the first progressive zone, up to 5.
    pub number: u8,
    /// First euro of the zone.
    pub from_eur: i128,
    /// Value of `t` is zero at this euro.
    pub base_eur: i128,
    /// Size of one unit of `t`, in euros.
    pub step_eur: i128,
    /// Coefficient `a` in hundredths of a euro.
    pub quadratic_h: i128,
    /// Coefficient `b` in hundredths of a euro.
    pub linear_h: i128,
    /// Coefficient `c` in hundredths of a euro.
    pub constant_h: i128,
}

/// A fraction of euros, with the numerator and denominator kept as integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    /// Numerator, in euros times the denominator.
    pub numerator: i128,
    /// Denominator, always positive.
    pub denominator: i128,
}

impl Fraction {
    /// Zero euros.
    pub const ZERO: Fraction = Fraction {
        numerator: 0,
        denominator: 1,
    };

    /// The value rounded down to whole euros (towards negative infinity).
    #[must_use]
    pub fn floor(self) -> i128 {
        self.numerator.div_euclid(self.denominator)
    }
}

/// Where a figure comes from: the § citation, the source it was taken from, and the archived copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Source {
    /// Statute citation of the tariff.
    pub citation: &'static str,
    /// Source the tariff was taken from.
    pub source: &'static str,
    /// Archived copy of the source, as a link to `docs/references`.
    pub reference: &'static str,
}

impl From<&params::Source> for Source {
    fn from(source: &params::Source) -> Self {
        Self {
            citation: source.citation,
            source: source.source,
            reference: source.reference,
        }
    }
}

/// The tariff steps for one basis: the zone (`None` in the zero zone), the unrounded tax, and the
/// rounded tax. Shared by [`explain`] and [`assess`], so they cannot disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BasisTax {
    zone: Option<usize>,
    tariff_basis_eur: i128,
    unrounded: Fraction,
    tax_eur: i128,
}

fn taxable_euros(income: Cents) -> Result<i128, Error> {
    Some(income)
        .filter(|income| income.get() >= 0)
        .map(Cents::whole_euros)
        .ok_or(Error::NegativeTaxableIncome)
}

fn filing_basis_eur(filing: FilingStatus, taxable_income_eur: i128) -> i128 {
    match filing {
        FilingStatus::Single => taxable_income_eur,
        FilingStatus::Joint => taxable_income_eur / 2,
    }
}

fn income_tax_eur(filing: FilingStatus, tax_on_basis_eur: i128) -> Option<i128> {
    match filing {
        FilingStatus::Single => Some(tax_on_basis_eur),
        FilingStatus::Joint => tax_on_basis_eur.checked_mul(2),
    }
}

/// Tax on `basis_eur` under `zones`, after the table step of the year if it has one. The zero zone
/// is decided on the basis itself, and the zones on the stepped basis. `None` when the stepped basis
/// is above the allowance and no zone covers it, or when the computation does not fit in `i128`.
fn basis_tax(
    zones: &[Zone],
    allowance_eur: i128,
    table: Option<&params::TableStep>,
    basis_eur: i128,
) -> Option<BasisTax> {
    if basis_eur <= allowance_eur {
        return Some(BasisTax {
            zone: None,
            tariff_basis_eur: basis_eur,
            unrounded: Fraction::ZERO,
            tax_eur: 0,
        });
    }
    zone_for(zones, table_basis(table, basis_eur)?)
}

/// The § 32a Abs. 2 table step: the basis rounded down to a multiple of the width, then raised by the
/// added amount. Without a table step the basis is unchanged.
fn table_basis(table: Option<&params::TableStep>, basis_eur: i128) -> Option<i128> {
    match table {
        None => Some(basis_eur),
        Some(step) => basis_eur
            .div_euclid(step.width_eur)
            .checked_mul(step.width_eur)?
            .checked_add(step.added_eur),
    }
}

fn zone_for(zones: &[Zone], basis_eur: i128) -> Option<BasisTax> {
    zones
        .iter()
        .enumerate()
        .rev()
        .find(|(_, zone)| basis_eur >= zone.from_eur)
        .and_then(|(index, zone)| {
            zone_fraction(zone, basis_eur).map(|unrounded| BasisTax {
                zone: Some(index),
                tariff_basis_eur: basis_eur,
                unrounded,
                tax_eur: unrounded.floor(),
            })
        })
}

/// `a·t² + b·t + c` with `t = (x − base) / step`, as one fraction of euros.
///
/// The coefficients are in hundredths, so with `d = x − base` the numerator is
/// `quadratic_h·d² + linear_h·d·step + constant_h·step²` and the denominator is `100·step²`.
/// Every product is checked, so an overflow is reported as `None`, never as a wrong tax.
fn zone_fraction(zone: &Zone, basis_eur: i128) -> Option<Fraction> {
    let d = basis_eur.checked_sub(zone.base_eur)?;
    let s = zone.step_eur;
    let terms = [
        zone.quadratic_h.checked_mul(d)?.checked_mul(d)?,
        zone.linear_h.checked_mul(d)?.checked_mul(s)?,
        zone.constant_h.checked_mul(s)?.checked_mul(s)?,
    ];
    let numerator = terms.into_iter().try_fold(0, i128::checked_add)?;
    let denominator = s.checked_mul(s)?.checked_mul(100)?;
    Some(Fraction {
        numerator,
        denominator,
    })
}

fn zone_step(index: usize, zone: &Zone) -> ZoneStep {
    ZoneStep {
        // Zones per year are few, so the fallback is never taken.
        number: u8::try_from(index + 2).unwrap_or(u8::MAX),
        from_eur: zone.from_eur,
        base_eur: zone.base_eur,
        step_eur: zone.step_eur,
        quadratic_h: zone.quadratic_h,
        linear_h: zone.linear_h,
        constant_h: zone.constant_h,
    }
}

/// Derivative of the zone's tax with respect to the basis, as a fraction of one euro.
#[expect(
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    reason = "the marginal rate is an f64 for display, and no tax figure is computed from it"
)]
fn marginal_rate(zone: &Zone, basis_eur: i128) -> f64 {
    let t = (basis_eur - zone.base_eur) as f64 / zone.step_eur as f64;
    let quadratic = zone.quadratic_h as f64 / 100.0;
    let linear = zone.linear_h as f64 / 100.0;
    (2.0 * quadratic * t + linear) / zone.step_eur as f64
}

#[expect(
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    reason = "the average rate is an f64 for display, and no tax figure is computed from it"
)]
fn ratio(numerator_eur: i128, denominator_eur: i128) -> f64 {
    Some(denominator_eur)
        .filter(|denominator| *denominator > 0)
        .map_or(0.0, |denominator| numerator_eur as f64 / denominator as f64)
}

#[cfg(kani)]
mod verification {
    use super::*;

    fn tax_on(zones: &[Zone], allowance_eur: i128, basis_eur: i128) -> Option<i128> {
        basis_tax(zones, allowance_eur, None, basis_eur).map(|basis| basis.tax_eur)
    }

    // Bounds keep the proofs tractable. Overflow is handled by checked arithmetic, and the
    // unit tests run the largest values.
    const BOUND_EUR: i128 = 1_000_000_000_000;
    const BOUND_COEFFICIENT: i128 = 1_000_000;

    fn any_zone() -> Zone {
        let zone = Zone {
            from_eur: kani::any(),
            base_eur: kani::any(),
            step_eur: kani::any(),
            quadratic_h: kani::any(),
            linear_h: kani::any(),
            constant_h: kani::any(),
        };
        kani::assume(zone.base_eur >= 0 && zone.base_eur <= BOUND_EUR);
        kani::assume(zone.step_eur >= 1 && zone.step_eur <= 10_000);
        kani::assume(
            zone.quadratic_h >= -(BOUND_COEFFICIENT) && zone.quadratic_h <= BOUND_COEFFICIENT,
        );
        kani::assume(zone.linear_h >= -(BOUND_COEFFICIENT) && zone.linear_h <= BOUND_COEFFICIENT);
        kani::assume(
            zone.constant_h >= -(BOUND_COEFFICIENT) && zone.constant_h <= BOUND_COEFFICIENT,
        );
        zone
    }

    // Up to the basic allowance the tax is zero, whatever the zones are.
    #[kani::proof]
    fn zero_tax_up_to_the_allowance_for_any_zone() {
        let zone = any_zone();
        let allowance_eur: i128 = kani::any();
        let basis_eur: i128 = kani::any();
        kani::assume(allowance_eur >= 0 && allowance_eur <= BOUND_EUR);
        kani::assume(basis_eur <= allowance_eur && basis_eur >= -BOUND_EUR);

        assert_eq!(tax_on(&[zone], allowance_eur, basis_eur), Some(0));
    }

    // The table step of § 32a Abs. 2 (2002 text): 36 € blocks, plus 18 €. The width is fixed to the statute's
    // value, because a symbolic width makes the solver time out. The unit tests cover other widths.
    // The stepped basis stays within its block, and never decreases for a higher basis.
    #[kani::proof]
    fn table_step_keeps_the_basis_within_its_block_and_monotone() {
        let step = params::TableStep {
            width_eur: 36,
            added_eur: 18,
        };
        let first: i128 = kani::any();
        let second: i128 = kani::any();
        kani::assume(first >= 0 && first <= BOUND_EUR);
        kani::assume(second >= first && second <= BOUND_EUR);

        let low = table_basis(Some(&step), first).unwrap();
        let high = table_basis(Some(&step), second).unwrap();
        assert!(low - first <= 18 && low - first > 18 - 36);
        assert!(low <= high);
    }

    // Above the allowance with no zones, there is no tax to compute, so the result is an error.
    #[kani::proof]
    fn above_the_allowance_without_zones_there_is_no_tax() {
        let allowance_eur: i128 = kani::any();
        let basis_eur: i128 = kani::any();
        kani::assume(allowance_eur >= 0 && allowance_eur <= BOUND_EUR);
        kani::assume(basis_eur > allowance_eur && basis_eur <= BOUND_EUR);

        assert_eq!(tax_on(&[], allowance_eur, basis_eur), None);
    }

    // A zone fraction always has a positive denominator, so its floor is a well-defined tax.
    #[kani::proof]
    fn zone_fraction_has_a_positive_denominator() {
        let zone = any_zone();
        let basis_eur: i128 = kani::any();
        kani::assume(basis_eur >= 0 && basis_eur <= BOUND_EUR);

        if let Some(fraction) = zone_fraction(&zone, basis_eur) {
            assert!(fraction.denominator > 0);
        }
    }

    // Joint tax is exactly twice the tax on the basis, and single tax is the tax on the basis.
    #[kani::proof]
    fn joint_tax_is_twice_the_basis_tax() {
        let tax_eur: i128 = kani::any();
        kani::assume(tax_eur >= 0 && tax_eur <= BOUND_EUR);

        assert_eq!(income_tax_eur(FilingStatus::Single, tax_eur), Some(tax_eur));
        assert_eq!(
            income_tax_eur(FilingStatus::Joint, tax_eur),
            Some(2 * tax_eur)
        );
    }

    // The joint basis is half the income, rounded down, so it never exceeds half of it.
    #[kani::proof]
    fn joint_basis_is_at_most_half_the_income() {
        let income_eur: i128 = kani::any();
        kani::assume(income_eur >= 0 && income_eur <= BOUND_EUR);

        let basis = filing_basis_eur(FilingStatus::Joint, income_eur);
        assert!(basis >= 0);
        assert!(2 * basis <= income_eur);
    }

    // Negative income is rejected, and only negative income is.
    #[kani::proof]
    fn only_negative_income_is_rejected() {
        let cents: i128 = kani::any();
        kani::assume(cents >= -(BOUND_EUR * 100) && cents <= BOUND_EUR * 100);

        let result = taxable_euros(Cents::new(cents));
        if cents < 0 {
            assert_eq!(result, Err(Error::NegativeTaxableIncome));
        } else {
            assert_eq!(result, Ok(cents / 100));
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    reason = "the rate tests compare f64 display values, with a tolerance where they differ"
)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // A small fixture tariff. The numbers are sample data, not statute values: the allowance is
    // 1 000 €, zone 2 is quadratic with `a = 100`, and zone 3 is linear and joins zone 2 at 2 000 €.
    const ALLOWANCE_EUR: i128 = 1_000;

    fn fixture_zones() -> Vec<Zone> {
        vec![
            Zone {
                from_eur: 1_001,
                base_eur: 1_000,
                step_eur: 10_000,
                quadratic_h: 10_000,
                linear_h: 0,
                constant_h: 0,
            },
            Zone {
                from_eur: 2_001,
                base_eur: 0,
                step_eur: 1,
                quadratic_h: 0,
                linear_h: 50,
                constant_h: -99_950,
            },
        ]
    }

    fn tax_on(zones: &[Zone], allowance_eur: i128, basis_eur: i128) -> Option<i128> {
        basis_tax(zones, allowance_eur, None, basis_eur).map(|basis| basis.tax_eur)
    }

    fn fixture_tax(basis_eur: i128) -> Option<i128> {
        tax_on(&fixture_zones(), ALLOWANCE_EUR, basis_eur)
    }

    #[test]
    fn fixture_is_zero_up_to_the_allowance() {
        assert_eq!(fixture_tax(0), Some(0));
        assert_eq!(fixture_tax(ALLOWANCE_EUR), Some(0));
    }

    #[test]
    fn fixture_rounds_the_zone_down_after_the_sum() {
        // 100 · (400 / 10 000)² = 0.16 €, which rounds down to 0 €.
        assert_eq!(fixture_tax(1_400), Some(0));
        // 100 · (1 000 / 10 000)² = 1 €, exactly.
        assert_eq!(fixture_tax(2_000), Some(1));
    }

    #[test]
    fn fixture_zones_join_within_one_euro() {
        assert_eq!(fixture_tax(2_000), Some(1));
        assert_eq!(fixture_tax(2_001), Some(1));
    }

    #[test]
    fn fixture_linear_zone_applies_its_own_coefficients() {
        // 0.5 · 3 000 − 999.5 = 500.5, which rounds down to 500 €.
        assert_eq!(fixture_tax(3_000), Some(500));
    }

    #[test]
    fn fixture_without_zones_is_an_error_above_the_allowance() {
        assert_eq!(tax_on(&[], ALLOWANCE_EUR, ALLOWANCE_EUR + 1), None);
        assert_eq!(tax_on(&[], ALLOWANCE_EUR, ALLOWANCE_EUR), Some(0));
    }

    #[test]
    fn overflow_is_reported_as_none() {
        let zone = Zone {
            from_eur: 1,
            base_eur: 0,
            step_eur: 1,
            quadratic_h: i128::MAX,
            linear_h: 0,
            constant_h: 0,
        };
        assert_eq!(tax_on(&[zone], 0, 2), None);
    }

    #[test]
    fn negative_income_is_rejected_before_the_tariff() {
        assert_eq!(
            taxable_euros(Cents::new(-1)),
            Err(Error::NegativeTaxableIncome)
        );
        assert_eq!(taxable_euros(Cents::new(0)), Ok(0));
    }

    #[test]
    fn filing_basis_halves_joint_income_rounding_down() {
        assert_eq!(filing_basis_eur(FilingStatus::Single, 7), 7);
        assert_eq!(filing_basis_eur(FilingStatus::Joint, 7), 3);
    }

    #[test]
    fn ratio_is_zero_for_zero_income() {
        assert_eq!(ratio(0, 0), 0.0);
        assert_eq!(ratio(1, 4), 0.25);
    }

    // Every year: the zero zone ends at the basic allowance, and the zone data starts right after.
    #[test]
    fn every_year_is_zero_up_to_its_basic_allowance() {
        for year in TaxYear::ALL {
            let allowance = basic_allowance_eur(year);
            let at_limit = Unit::single(allowance * 100);
            assert_eq!(assess(year, &at_limit).unwrap().income_tax, Cents::new(0));
        }
    }

    // Tax never jumps by more than one euro at a zone start, so the zones join, as § 32a requires. With
    // a table step the basis moves by a whole block at a zone start, so the jump may be one block.
    #[test]
    fn every_zone_joins_the_previous_one() {
        for year in TaxYear::ALL {
            let largest_jump = params::table_step_for(year).map_or(1, |step| step.width_eur);
            for zone in tariff_zones(year) {
                let below = Unit::single((zone.from_eur - 1) * 100);
                let at = Unit::single(zone.from_eur * 100);
                let (Ok(below), Ok(at)) = (assess(year, &below), assess(year, &at)) else {
                    continue;
                };
                let jump = at.income_tax.whole_euros() - below.income_tax.whole_euros();
                assert!(
                    (0..=largest_jump).contains(&jump),
                    "{year:?} at {}",
                    zone.from_eur
                );
            }
        }
    }

    // VZ 2003 uses the 2002 tariff (BMF tariff history, section 15, and the § 32a text of 2002). The basis is
    // rounded to 36 € blocks before the zones apply, so the tax is flat inside a block: 7 236 to 7 271 € pay 10 €.
    #[test]
    fn year_2003_rounds_the_basis_to_its_block_before_the_zones() {
        let tax = |euros: i128| {
            assess(TaxYear::Y2003, &Unit::single(euros * 100))
                .unwrap()
                .income_tax
                .whole_euros()
        };
        assert_eq!(tax(7_235), 0);
        assert_eq!(tax(7_236), 10);
        assert_eq!(tax(7_253), 10);
        assert_eq!(tax(7_272), 17);
        assert_eq!(tax(9_251), 436);
        assert_eq!(tax(9_252), 444);
        assert_eq!(tax(55_007), 16_798);
        assert_eq!(tax(55_008), 16_815);
        // Joint filers: the half, 7 236 €, is the basis.
        let joint = assess(TaxYear::Y2003, &Unit::joint(14_472 * 100)).unwrap();
        assert_eq!(joint.income_tax, Cents::from_euros(20));
    }

    // Each zone's numbers agree with its own formula, taken from the rule data.
    #[test]
    fn explanation_zone_contains_the_basis() {
        for year in TaxYear::ALL {
            for zone in tariff_zones(year) {
                let unit = Unit::single(zone.from_eur * 100);
                let explanation = explain(year, &unit).unwrap();
                assert_eq!(
                    explanation.zone,
                    Some(zone),
                    "{year:?} at {}",
                    zone.from_eur
                );
            }
        }
    }

    // Helpers that build filing units from cents.
    struct Unit;

    impl Unit {
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
    }

    fn year_index() -> impl Strategy<Value = usize> {
        0..TaxYear::ALL.len()
    }

    fn years_with_table_step() -> Vec<TaxYear> {
        TaxYear::ALL
            .into_iter()
            .filter(|year| params::table_step_for(*year).is_some())
            .collect()
    }

    fn years_with_zones() -> Vec<TaxYear> {
        TaxYear::ALL
            .into_iter()
            .filter(|year| !tariff_zones(*year).is_empty())
            .collect()
    }

    proptest! {
        // Cents below one euro never change the tax: taxable income is rounded down first.
        #[test]
        fn cents_do_not_change_the_tax(index in year_index(), euros in 0..2_000_000i128, cents in 0..100i128) {
            let year = TaxYear::ALL[index];
            let whole = assess(year, &Unit::single(euros * 100));
            let with_cents = assess(year, &Unit::single(euros * 100 + cents));
            prop_assume!(whole.is_ok());
            prop_assert_eq!(with_cents, whole);
        }

        // More income never means less tax, in any year that has a tariff, for both filings.
        #[test]
        fn tax_never_decreases_with_income(
            index in year_index(),
            a in 0..=2_000_000i128,
            b in 0..=2_000_000i128,
            joint_filing in any::<bool>(),
        ) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            let (low, high) = (a.min(b), a.max(b));
            let unit = |euros: i128| if joint_filing { Unit::joint(euros * 100) } else { Unit::single(euros * 100) };
            let low_tax = assess(year, &unit(low)).unwrap().income_tax;
            let high_tax = assess(year, &unit(high)).unwrap().income_tax;
            prop_assert!(low_tax <= high_tax);
        }

        // Without a table step, one more euro of income adds 0 € or 1 € of tax, since every marginal rate is below 100 %.
        #[test]
        fn one_more_euro_adds_at_most_one_euro_of_tax(index in year_index(), euros in 0..2_000_000i128) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty() && params::table_step_for(year).is_none());
            let tax = |e: i128| assess(year, &Unit::single(e * 100)).unwrap().income_tax.whole_euros();
            let step = tax(euros + 1) - tax(euros);
            prop_assert!(step == 0 || step == 1);
        }

        // With a table step the tax is flat inside each block, and a whole block adds at most one euro per euro.
        #[test]
        fn tax_is_flat_inside_each_table_block(index in 0..64usize, euros in 0..2_000_000i128) {
            let years = years_with_table_step();
            let year = years[index % years.len()];
            let width = params::table_step_for(year).unwrap().width_eur;
            let tax = |e: i128| assess(year, &Unit::single(e * 100)).unwrap().income_tax.whole_euros();
            let block_start = euros - euros.rem_euclid(width);
            prop_assert_eq!(tax(euros), tax(block_start));
            let rise = tax(block_start + width) - tax(block_start);
            prop_assert!((0..=width).contains(&rise), "{year:?} at {euros}: a block rises by {rise}");
        }

        // Joint tax is exactly twice the single tax on half of the income, rounded down.
        #[test]
        fn joint_tax_is_twice_the_single_tax_on_half_the_income(index in year_index(), euros in 0..2_000_000i128) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            let joint_tax = assess(year, &Unit::joint(euros * 100)).unwrap().income_tax;
            let half_tax = assess(year, &Unit::single((euros / 2) * 100)).unwrap().income_tax;
            prop_assert_eq!(joint_tax, Cents::from_euros(2 * half_tax.whole_euros()));
        }

        // Up to the allowance, with any cents part, the tax is zero in every year.
        #[test]
        fn zero_tax_up_to_the_allowance_for_any_cents(index in year_index(), cents in 0..100i128, offset in 0..=1_000i128) {
            let year = TaxYear::ALL[index];
            let allowance = basic_allowance_eur(year);
            let euros = (allowance - offset).max(0);
            let result = assess(year, &Unit::single(euros * 100 + cents)).unwrap();
            prop_assert_eq!(result.income_tax, Cents::new(0));
        }

        // The explanation and the assessment agree, and the unrounded tax floors to the rounded one.
        #[test]
        fn explanation_agrees_with_the_assessment(index in year_index(), euros in 0..2_000_000i128, joint_filing in any::<bool>()) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            let unit = if joint_filing { Unit::joint(euros * 100) } else { Unit::single(euros * 100) };
            let explanation = explain(year, &unit).unwrap();
            prop_assert_eq!(Cents::from_euros(explanation.income_tax_eur), assess(year, &unit).unwrap().income_tax);
            prop_assert_eq!(explanation.unrounded_basis_tax.floor(), explanation.basis_tax_eur);
            prop_assert!(explanation.marginal_rate.is_finite() && (0.0..1.0).contains(&explanation.marginal_rate));
            prop_assert!(explanation.average_rate.is_finite() && (0.0..1.0).contains(&explanation.average_rate));
        }

        // Any i128 cents value, single or joint, in any year, returns Ok or Err and never panics.
        #[test]
        fn assess_and_explain_never_panic(index in year_index(), cents in any::<i128>(), joint_filing in any::<bool>()) {
            let year = TaxYear::ALL[index];
            let unit = if joint_filing { Unit::joint(cents) } else { Unit::single(cents) };
            let _ = assess(year, &unit);
            let _ = explain(year, &unit);
        }

        // The floor of a fraction is the mathematical floor: q·den ≤ num < (q + 1)·den.
        #[test]
        fn fraction_floor_is_the_mathematical_floor(numerator in -1_000_000_000_000_000_000_000_000_000_000i128..=1_000_000_000_000_000_000_000_000_000_000i128, denominator in 1..=1_000_000_000i128) {
            let fraction = Fraction { numerator, denominator };
            let q = fraction.floor();
            prop_assert!(q * denominator <= numerator);
            prop_assert!(numerator < (q + 1) * denominator);
        }

        // Negative income is always an input error, never a zero tax.
        #[test]
        fn negative_income_is_always_rejected(index in year_index(), cents in i128::MIN..0) {
            let year = TaxYear::ALL[index];
            prop_assert_eq!(assess(year, &Unit::single(cents)).unwrap_err(), Error::NegativeTaxableIncome);
        }
    }

    // Fixture: a zone with a = 100 € and step 10 000 € has marginal rate 2·a·t / step, with t = (x − base) / step.
    #[test]
    fn fixture_marginal_rate_is_the_derivative_of_the_zone() {
        let zone = fixture_zones()[0];
        assert_eq!(marginal_rate(&zone, 1_000), 0.0);
        // t = 0.1, so 2 · 100 · 0.1 / 10 000 = 0.002.
        assert!((marginal_rate(&zone, 2_000) - 0.002).abs() < 1e-12);
    }

    // Fixture: a linear zone has the constant rate of its linear coefficient.
    #[test]
    fn fixture_marginal_rate_of_a_linear_zone_is_its_slope() {
        let zone = fixture_zones()[1];
        assert!((marginal_rate(&zone, 3_000) - 0.5).abs() < 1e-12);
        assert!((marginal_rate(&zone, 9_000) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn zero_zone_has_zero_rates() {
        for year in TaxYear::ALL {
            let allowance = basic_allowance_eur(year);
            let explanation = explain(year, &Unit::single(allowance * 100)).unwrap();
            assert_eq!(explanation.marginal_rate, 0.0, "{year:?}");
            assert_eq!(explanation.average_rate, 0.0, "{year:?}");
        }
    }

    // The marginal rate is the tax on the next euro. Within one zone the unrounded tax is a quadratic
    // in income, so the central difference over one euro on each side equals the derivative exactly.
    proptest! {
        #[test]
        fn marginal_rate_matches_the_central_difference_of_the_unrounded_tax(index in year_index(), euros in 40..2_000_000i128) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            // With a table step the basis moves by a block per block, so the difference spans one block.
            let width = params::table_step_for(year).map_or(1, |step| step.width_eur);
            let at = |e: i128| explain(year, &Unit::single(e * 100)).unwrap();
            let (below, here, above) = (at(euros - width), at(euros), at(euros + width));
            prop_assume!(below.zone == here.zone && here.zone == above.zone && here.zone.is_some());
            let unrounded = |e: &Explanation| e.unrounded_basis_tax.numerator as f64 / e.unrounded_basis_tax.denominator as f64;
            let central = (unrounded(&above) - unrounded(&below)) / (2.0 * width as f64);
            prop_assert!((central - here.marginal_rate).abs() < 1e-8, "{year:?} at {euros}: {central} vs {}", here.marginal_rate);
        }

        // A joint filer's marginal rate is the single filer's rate on half the income.
        #[test]
        fn joint_marginal_rate_is_the_single_rate_on_half_the_income(index in year_index(), euros in 0..2_000_000i128) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            let joint = explain(year, &Unit::joint(euros * 100)).unwrap();
            let single = explain(year, &Unit::single((euros / 2) * 100)).unwrap();
            prop_assert_eq!(joint.marginal_rate, single.marginal_rate);
        }

        // The average rate is the income tax over the taxable income.
        #[test]
        fn average_rate_is_the_tax_over_the_income(index in year_index(), euros in 1..2_000_000i128, joint_filing in any::<bool>()) {
            let year = TaxYear::ALL[index];
            prop_assume!(!tariff_zones(year).is_empty());
            let unit = if joint_filing { Unit::joint(euros * 100) } else { Unit::single(euros * 100) };
            let explanation = explain(year, &unit).unwrap();
            let expected = explanation.income_tax_eur as f64 / explanation.taxable_income_eur as f64;
            prop_assert!((explanation.average_rate - expected).abs() < 1e-12);
        }
    }

    // A year without zones reports an error above its allowance, and only there.
    #[test]
    fn years_without_zones_report_out_of_range_above_the_allowance() {
        for year in TaxYear::ALL
            .into_iter()
            .filter(|y| tariff_zones(*y).is_empty())
        {
            let allowance = basic_allowance_eur(year);
            assert_eq!(
                assess(year, &Unit::single(allowance * 100))
                    .unwrap()
                    .income_tax,
                Cents::new(0)
            );
            assert!(matches!(
                assess(year, &Unit::single((allowance + 1) * 100)),
                Err(Error::OutsideImplementedRange { .. })
            ));
        }
    }

    #[test]
    fn at_least_one_year_has_zones() {
        assert_ne!(years_with_zones(), Vec::<TaxYear>::new());
    }
}
