// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use crate::params::{self, Zone};
use crate::{Assessment, Cents, Error, FilingStatus, FilingUnit, TaxYear};

/// Assesses income tax for one filing unit in `year`.
///
/// Taxable income is given in cents. Income tax is in cents, always a whole number of euros.
/// Returns [`Error::NegativeTaxableIncome`] for negative income. Every tax year listed in
/// [`TaxYear`] has its tariff.
///
/// # Examples
///
/// ```
/// use fiskus::{assess, Cents, FilingStatus, FilingUnit, TaxYear};
///
/// let unit = FilingUnit {
///     filing: FilingStatus::Single,
///     taxable_income: Cents::from_euros(30_000),
/// };
/// let assessment = assess(TaxYear::Y2026, &unit).unwrap();
/// assert_eq!(assessment.income_tax, Cents::from_euros(4_217));
/// ```
pub fn assess(year: TaxYear, unit: &FilingUnit) -> Result<Assessment, Error> {
    let explanation = explain(year, unit)?;
    Ok(Assessment {
        taxable_income: Cents::from_euros(explanation.taxable_income_eur),
        income_tax: Cents::from_euros(explanation.income_tax_eur),
    })
}

/// Explains how the income tax for one filing unit in `year` is computed.
///
/// The result has the same tax as [`assess`], plus the zone used, the unrounded tax, and the
/// citation and source of the year's tariff. Amounts are whole euros, and `unrounded_basis_tax`
/// is a fraction of euros.
///
/// # Examples
///
/// ```
/// use fiskus::{explain, Cents, FilingStatus, FilingUnit, TaxYear};
///
/// let unit = FilingUnit {
///     filing: FilingStatus::Single,
///     taxable_income: Cents::from_euros(30_000),
/// };
/// let explanation = explain(TaxYear::Y2026, &unit).unwrap();
/// assert_eq!(explanation.income_tax_eur, 4_217);
/// assert_eq!(explanation.zone.unwrap().number, 3);
/// ```
pub fn explain(year: TaxYear, unit: &FilingUnit) -> Result<Explanation, Error> {
    if unit.taxable_income.get() < 0 {
        return Err(Error::NegativeTaxableIncome);
    }

    let taxable_income_eur = unit.taxable_income.whole_euros();
    let basis_eur = filing_basis_eur(unit.filing, taxable_income_eur);
    let basic_allowance_eur = basic_allowance_eur(year);
    let zones = params::tariff_for(year);
    let breakdown = basis_breakdown(zones, basic_allowance_eur, basis_eur);
    let source = params::tariff_source_for(year);

    let income_tax_eur = income_tax_eur(unit.filing, breakdown.tax);
    let average_rate = if taxable_income_eur > 0 {
        income_tax_eur as f64 / taxable_income_eur as f64
    } else {
        0.0
    };
    let marginal_rate = match breakdown.zone {
        Some(index) => marginal_rate(Some(&zones[index]), basis_eur),
        None => 0.0,
    };

    Ok(Explanation {
        year,
        filing: unit.filing,
        taxable_income_eur,
        basis_eur,
        basic_allowance_eur,
        zone: breakdown.zone.map(|index| zone_step(index, &zones[index])),
        unrounded_basis_tax: Fraction {
            numerator: breakdown.numerator,
            denominator: breakdown.denominator,
        },
        basis_tax_eur: breakdown.tax,
        income_tax_eur,
        average_rate,
        marginal_rate,
        citation: source.citation,
        source: source.source,
        reference: source.reference,
    })
}

/// The progressive zones of the tariff of `year`, from zone 2 upward, with their coefficients.
///
/// Zone 1 is the basic allowance (zero tax); it is returned by [`basic_allowance_eur`]. Amounts
/// are whole euros. Coefficients are in hundredths of a euro, see [`ZoneStep`].
///
/// # Examples
///
/// ```
/// use fiskus::{tariff_zones, TaxYear};
///
/// let zones = tariff_zones(TaxYear::Y2026);
/// assert_eq!(zones[0].number, 2);
/// assert_eq!(zones[0].from_eur, 12_349);
/// ```
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
/// assert_eq!(basic_allowance_eur(TaxYear::Y2026), 12_348);
/// ```
pub fn basic_allowance_eur(year: TaxYear) -> i128 {
    params::for_year(year).basic_allowance.value_eur
}

/// Income tax with its steps: the zone, the unrounded tax, and the rounded result.
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
    /// Average tax rate: income tax divided by taxable income.
    pub average_rate: f64,
    /// Marginal tax rate: tax on the next euro of income, from the zone's derivative.
    pub marginal_rate: f64,
    /// Statute citation of the year's tariff.
    pub citation: &'static str,
    /// Source the year's tariff was taken from.
    pub source: &'static str,
    /// Archived copy of the source, in `docs/references`.
    pub reference: &'static str,
}

/// One progressive zone of the tariff.
///
/// The tax on a basis `x` inside the zone is `a·t² + b·t + c`, with `t = (x − base_eur) / step_eur`.
/// The coefficients `a`, `b` and `c` are `quadratic_h`, `linear_h` and `constant_h` in hundredths
/// of a euro, so `quadratic_h = 91451` means `a = 914.51`. Linear zones have `quadratic_h = 0`
/// and `step_eur = 1`.
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
    /// The value rounded down to whole euros.
    pub fn floor(self) -> i128 {
        self.numerator.div_euclid(self.denominator)
    }
}

fn zone_step(index: usize, zone: &Zone) -> ZoneStep {
    ZoneStep {
        number: (index + 2) as u8,
        from_eur: zone.from_eur,
        base_eur: zone.base_eur,
        step_eur: zone.step_eur,
        quadratic_h: zone.quadratic_h,
        linear_h: zone.linear_h,
        constant_h: zone.constant_h,
    }
}

fn filing_basis_eur(filing: FilingStatus, taxable_income_eur: i128) -> i128 {
    match filing {
        FilingStatus::Single => taxable_income_eur,
        FilingStatus::Joint => taxable_income_eur / 2,
    }
}

/// Marginal tax rate of the zone at `basis`: the derivative of its tax, as a fraction of one euro.
fn marginal_rate(zone: Option<&Zone>, basis_eur: i128) -> f64 {
    match zone {
        None => 0.0,
        Some(zone) => {
            let t = (basis_eur - zone.base_eur) as f64 / zone.step_eur as f64;
            let a = zone.quadratic_h as f64 / 100.0;
            let b = zone.linear_h as f64 / 100.0;
            (2.0 * a * t + b) / zone.step_eur as f64
        }
    }
}

fn income_tax_eur(filing: FilingStatus, basis_tax_eur: i128) -> i128 {
    match filing {
        FilingStatus::Single => basis_tax_eur,
        FilingStatus::Joint => 2 * basis_tax_eur,
    }
}

/// The tariff steps for one basis: the zone index (`None` in the zero zone), the unrounded tax as
/// a fraction, and the rounded tax. Shared by [`explain`] and the tax itself, so they agree.
struct BasisBreakdown {
    zone: Option<usize>,
    numerator: i128,
    denominator: i128,
    tax: i128,
}

fn basis_breakdown(zones: &[Zone], basic_allowance_eur: i128, basis_eur: i128) -> BasisBreakdown {
    let zero = BasisBreakdown {
        zone: None,
        numerator: 0,
        denominator: 1,
        tax: 0,
    };
    if basis_eur <= basic_allowance_eur {
        return zero;
    }
    match zones
        .iter()
        .enumerate()
        .rev()
        .find(|(_, zone)| basis_eur >= zone.from_eur)
    {
        Some((index, zone)) => {
            let (numerator, denominator) = zone_fraction(zone, basis_eur);
            BasisBreakdown {
                zone: Some(index),
                numerator,
                denominator,
                tax: numerator.div_euclid(denominator),
            }
        }
        None => zero,
    }
}

/// Evaluates one zone exactly with integers. Returns the numerator and denominator of the tax
/// in euros, so that rounding down happens once, at the end of the zone.
fn zone_fraction(zone: &Zone, basis_eur: i128) -> (i128, i128) {
    let d = basis_eur - zone.base_eur;
    let s = zone.step_eur;
    let quadratic = if zone.quadratic_h == 0 {
        0
    } else {
        zone.quadratic_h * d * d
    };
    (
        quadratic + zone.linear_h * d * s + zone.constant_h * s * s,
        100 * s * s,
    )
}

#[cfg(kani)]
mod verification {
    use super::*;

    // 10^12 euros keeps the proof fast enough for CI. The no-panic harness covers every income
    // that Cents can represent.
    const BOUND_EUR: i128 = 1_000_000_000_000;

    // Up to the basic allowance the tariff is zero, for any allowance and income within the bound.
    #[kani::proof]
    fn zero_zone_is_exact() {
        let basic_allowance_eur: i128 = kani::any();
        let basis_eur: i128 = kani::any();
        kani::assume(basic_allowance_eur >= 0 && basic_allowance_eur <= BOUND_EUR);
        kani::assume(basis_eur >= 0 && basis_eur <= basic_allowance_eur);

        assert_eq!(
            basis_breakdown(
                params::tariff_for(TaxYear::Y2026),
                basic_allowance_eur,
                basis_eur
            )
            .tax,
            0
        );
    }

    // filing_basis_eur is non-negative for non-negative taxable income.
    #[kani::proof]
    fn filing_basis_non_negative() {
        const BOUND: i128 = 1_000_000_000_000;
        let taxable: i128 = kani::any();
        kani::assume(taxable >= 0 && taxable <= BOUND);
        let single = filing_basis_eur(FilingStatus::Single, taxable);
        let joint = filing_basis_eur(FilingStatus::Joint, taxable);
        assert!(single >= 0);
        assert!(joint >= 0);
    }

    // For any allowance within the bound and any income that Cents can hold, the 2026 tariff
    // returns a non-negative tax without panicking or overflowing.
    #[kani::proof]
    fn tariff_never_panics() {
        let basic_allowance_eur: i128 = kani::any();
        let basis_eur: i128 = kani::any();
        kani::assume(basic_allowance_eur >= 0 && basic_allowance_eur <= BOUND_EUR);
        kani::assume(basis_eur >= 0 && basis_eur <= i128::MAX / 100);

        let tax = basis_breakdown(
            params::tariff_for(TaxYear::Y2026),
            basic_allowance_eur,
            basis_eur,
        )
        .tax;
        assert!(tax >= 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Basic allowance (Grundfreibetrag) for VZ 2026 (§ 32a Abs. 1 Satz 2 Nr. 1 EStG).
    const BASIC_ALLOWANCE_EUR: i128 = 12_348;
    // Largest joint zvE still at zero tax: the half (12 348.50) rounds down to 12 348, and its tax is 0 €.
    const JOINT_ZERO_TAX_EUR: i128 = 24_711;

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

    fn tax_eur(year: TaxYear, unit: &FilingUnit) -> i128 {
        assess(year, unit).unwrap().income_tax.get() / 100
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

    // The first euro above the allowance adds 0,14 €, which rounds down to 0 €.
    #[test]
    fn single_first_euro_above_basic_allowance_rounds_to_zero_tax() {
        assert_eq!(
            tax_eur(TaxYear::Y2026, &single((BASIC_ALLOWANCE_EUR + 1) * 100)),
            0
        );
    }

    // Joint filer at 24 697 €: the Splitting half is 12 348 €, so tax is 0 (§ 32a Abs. 5 EStG).
    #[test]
    fn joint_is_tax_free_up_to_the_split_zone_boundary() {
        let result = assess(TaxYear::Y2026, &joint(24_697 * 100)).unwrap();
        assert_eq!(result.income_tax, Cents::new(0));
    }

    // Joint filer at 24 711 €: the half is 12 355 €, whose tax still rounds down to 0 €.
    #[test]
    fn joint_at_the_last_zero_tax_euro_is_still_tax_free() {
        assert_eq!(tax_eur(TaxYear::Y2026, &joint(JOINT_ZERO_TAX_EUR * 100)), 0);
    }

    // zvE below zero is an input error, not a tax of 0 (§ 2 Abs. 5 EStG).
    #[test]
    fn negative_taxable_income_is_rejected() {
        assert_eq!(
            assess(TaxYear::Y2026, &single(-1)).unwrap_err(),
            Error::NegativeTaxableIncome
        );
    }

    // Single filer at 15 000 €: zone 2 gives 435 €, from the PAP 2026 formula.
    #[test]
    fn single_second_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(15_000 * 100)), 435);
    }

    // Single filer at 17 799 € and 17 800 €: the zone 2 and zone 3 values join at the edge.
    #[test]
    fn single_zone_edge_between_second_and_third_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(17_799 * 100)), 1_034);
        assert_eq!(tax_eur(TaxYear::Y2026, &single(17_800 * 100)), 1_035);
    }

    // Single filer at 30 000 €: zone 3 gives 4 217 €, from the PAP 2026 formula.
    #[test]
    fn single_third_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(30_000 * 100)), 4_217);
    }

    // Single filer at 69 878 € and 69 879 €: zone 3 and zone 4 give the same 18 213 € at the edge.
    #[test]
    fn single_zone_edge_between_third_and_fourth_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(69_878 * 100)), 18_213);
        assert_eq!(tax_eur(TaxYear::Y2026, &single(69_879 * 100)), 18_213);
    }

    // Single filer at 100 000 €: zone 4 gives 30 864 €, from the PAP 2026 formula.
    #[test]
    fn single_fourth_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(100_000 * 100)), 30_864);
    }

    // Single filer at 277 825 € and 277 826 €: the top zone starts at 277 826 €, with 105 551 € tax.
    #[test]
    fn single_top_zone_matches_the_pap_formula() {
        assert_eq!(tax_eur(TaxYear::Y2026, &single(277_825 * 100)), 105_550);
        assert_eq!(tax_eur(TaxYear::Y2026, &single(277_826 * 100)), 105_551);
        assert_eq!(tax_eur(TaxYear::Y2026, &single(300_000 * 100)), 115_529);
    }

    // Joint filer at 100 000 €: the half is 50 000 €, taxed and doubled to 21 096 € (§ 32a Abs. 5 EStG).
    #[test]
    fn joint_doubles_the_tax_on_half_the_income() {
        assert_eq!(tax_eur(TaxYear::Y2026, &joint(100_000 * 100)), 21_096);
    }

    // Basic allowance (Grundfreibetrag) per verified VZ, with the law that set it; see docs/references/README.md.
    const VERIFIED_BASIC_ALLOWANCES: [(TaxYear, i128); 21] = [
        // VZ 2006 to 2008: value from the BMF withholding guidance (Programmablaufplan); setting law not yet traced.
        (TaxYear::Y2006, 7_664),
        (TaxYear::Y2007, 7_664),
        (TaxYear::Y2008, 7_664),
        // VZ 2009: financial stability act (StabSiG), BGBl. 2009 I S. 416.
        (TaxYear::Y2009, 7_834),
        // VZ 2010 to 2012: financial stability act (StabSiG) § 52 Abs. 41, kept by BGBl. 2013 I S. 283.
        (TaxYear::Y2010, 8_004),
        (TaxYear::Y2011, 8_004),
        (TaxYear::Y2012, 8_004),
        // VZ 2013: act to reduce cold progression (Gesetz zum Abbau der kalten Progression), BGBl. 2013 I S. 283.
        (TaxYear::Y2013, 8_130),
        // VZ 2014: same law as 2013.
        (TaxYear::Y2014, 8_354),
        // VZ 2015: act raising the basic allowance (KiGAnhG 2015), BGBl. 2015 I S. 1202.
        (TaxYear::Y2015, 8_472),
        // VZ 2016: same law as 2015.
        (TaxYear::Y2016, 8_652),
        // VZ 2017 and 2018: EU mutual assistance directive act (BGBl. 2016 I S. 3000).
        (TaxYear::Y2017, 8_820),
        (TaxYear::Y2018, 9_000),
        // VZ 2019 and 2020: family relief act (Familienentlastungsgesetz, BGBl. 2018 I S. 2210).
        (TaxYear::Y2019, 9_168),
        (TaxYear::Y2020, 9_408),
        // VZ 2021: second family relief act (BGBl. 2020 I S. 2616).
        (TaxYear::Y2021, 9_744),
        // VZ 2022: tax relief act 2022 (Steuerentlastungsgesetz 2022, BGBl. 2022 I S. 749).
        (TaxYear::Y2022, 10_347),
        // VZ 2023: inflation compensation act (Inflationsausgleichsgesetz, BGBl. 2022 I S. 2230).
        (TaxYear::Y2023, 10_908),
        // VZ 2024: act on the subsistence minimum 2024 (Gesetz zur steuerlichen Freistellung des Existenzminimums 2024), BGBl. 2024 I Nr. 386, Art. 1.
        (TaxYear::Y2024, 11_784),
        // VZ 2025: tax development act (Steuerfortentwicklungsgesetz, SteFeG), BGBl. 2024 I Nr. 449, Art. 1.
        (TaxYear::Y2025, 12_096),
        // VZ 2026: tax development act (Steuerfortentwicklungsgesetz, SteFeG), BGBl. 2024 I Nr. 449, Art. 2.
        (TaxYear::Y2026, 12_348),
    ];

    // Every year in the verified table is listed in TaxYear::ALL, and the other way round.
    #[test]
    fn verified_years_match_the_listed_years() {
        assert_eq!(
            VERIFIED_BASIC_ALLOWANCES.map(|(year, _)| year),
            TaxYear::ALL
        );
    }

    // The explanation gives the same tax as assess, in every year, for single and joint filers.
    #[test]
    fn explanation_agrees_with_assessment_for_every_year() {
        for year in TaxYear::ALL {
            for euros in [0, 12_349, 30_000, 100_000, 300_000] {
                for filing in [FilingStatus::Single, FilingStatus::Joint] {
                    let unit = FilingUnit {
                        filing,
                        taxable_income: Cents::from_euros(euros),
                    };
                    let explanation = explain(year, &unit).unwrap();
                    let assessment = assess(year, &unit).unwrap();
                    assert_eq!(
                        Cents::from_euros(explanation.income_tax_eur),
                        assessment.income_tax
                    );
                    assert_eq!(
                        explanation.unrounded_basis_tax.floor(),
                        explanation.basis_tax_eur
                    );
                }
            }
        }
    }

    // Every listed year is zero-tax up to its basic allowance, for single and joint filers.
    #[test]
    fn every_year_is_tax_free_up_to_its_basic_allowance() {
        for (year, allowance_eur) in VERIFIED_BASIC_ALLOWANCES {
            assert_eq!(
                tax_eur(year, &single(allowance_eur * 100)),
                0,
                "{year:?}: single"
            );
            assert_eq!(
                tax_eur(year, &joint((2 * allowance_eur + 1) * 100)),
                0,
                "{year:?}: joint"
            );
        }
    }

    // Every zone of every listed year joins the previous zone within one euro of tax.
    #[test]
    fn every_year_has_continuous_tax_at_its_zone_edges() {
        for year in TaxYear::ALL {
            for zone in &params::tariff_for(year)[1..] {
                let below = tax_eur(year, &single((zone.from_eur - 1) * 100));
                let at = tax_eur(year, &single(zone.from_eur * 100));
                assert!(
                    (at - below).abs() <= 1,
                    "{year:?}: jump at {}",
                    zone.from_eur
                );
            }
        }
    }

    // Independent calculation of the tariff (exact fractions, from the same tables) at sample incomes.
    // The rows are generated from rules/<year>.toml, not from this file.
    const ORACLE: [(TaxYear, i128, i128); 105] = [
        (TaxYear::Y2006, 20_000, 2_850),
        (TaxYear::Y2006, 40_000, 9_223),
        (TaxYear::Y2006, 100_000, 34_086),
        (TaxYear::Y2006, 250_000, 97_086),
        (TaxYear::Y2006, 300_000, 118_086),
        (TaxYear::Y2007, 20_000, 2_850),
        (TaxYear::Y2007, 40_000, 9_223),
        (TaxYear::Y2007, 100_000, 34_086),
        (TaxYear::Y2007, 250_000, 97_086),
        (TaxYear::Y2007, 300_000, 119_586),
        (TaxYear::Y2008, 20_000, 2_850),
        (TaxYear::Y2008, 40_000, 9_223),
        (TaxYear::Y2008, 100_000, 34_086),
        (TaxYear::Y2008, 250_000, 97_086),
        (TaxYear::Y2008, 300_000, 119_586),
        (TaxYear::Y2009, 20_000, 2_759),
        (TaxYear::Y2009, 40_000, 9_095),
        (TaxYear::Y2009, 100_000, 33_936),
        (TaxYear::Y2009, 250_000, 96_936),
        (TaxYear::Y2009, 300_000, 119_424),
        (TaxYear::Y2010, 20_000, 2_701),
        (TaxYear::Y2010, 40_000, 9_007),
        (TaxYear::Y2010, 100_000, 33_828),
        (TaxYear::Y2010, 250_000, 96_828),
        (TaxYear::Y2010, 300_000, 119_306),
        (TaxYear::Y2011, 20_000, 2_701),
        (TaxYear::Y2011, 40_000, 9_007),
        (TaxYear::Y2011, 100_000, 33_828),
        (TaxYear::Y2011, 250_000, 96_828),
        (TaxYear::Y2011, 300_000, 119_306),
        (TaxYear::Y2012, 20_000, 2_701),
        (TaxYear::Y2012, 40_000, 9_007),
        (TaxYear::Y2012, 100_000, 33_828),
        (TaxYear::Y2012, 250_000, 96_828),
        (TaxYear::Y2012, 300_000, 119_306),
        (TaxYear::Y2013, 20_000, 2_677),
        (TaxYear::Y2013, 40_000, 8_983),
        (TaxYear::Y2013, 100_000, 33_804),
        (TaxYear::Y2013, 250_000, 96_804),
        (TaxYear::Y2013, 300_000, 119_282),
        (TaxYear::Y2014, 20_000, 2_634),
        (TaxYear::Y2014, 40_000, 8_940),
        (TaxYear::Y2014, 100_000, 33_761),
        (TaxYear::Y2014, 250_000, 96_761),
        (TaxYear::Y2014, 300_000, 119_239),
        (TaxYear::Y2015, 20_000, 2_611),
        (TaxYear::Y2015, 40_000, 8_918),
        (TaxYear::Y2015, 100_000, 33_738),
        (TaxYear::Y2015, 250_000, 96_738),
        (TaxYear::Y2015, 300_000, 119_216),
        (TaxYear::Y2016, 20_000, 2_560),
        (TaxYear::Y2016, 40_000, 8_826),
        (TaxYear::Y2016, 100_000, 33_605),
        (TaxYear::Y2016, 250_000, 96_605),
        (TaxYear::Y2016, 300_000, 118_972),
        (TaxYear::Y2017, 20_000, 2_520),
        (TaxYear::Y2017, 40_000, 8_766),
        (TaxYear::Y2017, 100_000, 33_524),
        (TaxYear::Y2017, 250_000, 96_524),
        (TaxYear::Y2017, 300_000, 118_835),
        (TaxYear::Y2018, 20_000, 2_467),
        (TaxYear::Y2018, 40_000, 8_670),
        (TaxYear::Y2018, 100_000, 33_378),
        (TaxYear::Y2018, 250_000, 96_378),
        (TaxYear::Y2018, 300_000, 118_562),
        (TaxYear::Y2019, 20_000, 2_414),
        (TaxYear::Y2019, 40_000, 8_569),
        (TaxYear::Y2019, 100_000, 33_219),
        (TaxYear::Y2019, 250_000, 96_219),
        (TaxYear::Y2019, 300_000, 118_259),
        (TaxYear::Y2020, 20_000, 2_346),
        (TaxYear::Y2020, 40_000, 8_452),
        (TaxYear::Y2020, 100_000, 33_036),
        (TaxYear::Y2020, 250_000, 96_036),
        (TaxYear::Y2020, 300_000, 117_921),
        (TaxYear::Y2021, 20_000, 2_266),
        (TaxYear::Y2021, 40_000, 8_333),
        (TaxYear::Y2021, 100_000, 32_863),
        (TaxYear::Y2021, 250_000, 95_863),
        (TaxYear::Y2021, 300_000, 117_625),
        (TaxYear::Y2022, 20_000, 2_138),
        (TaxYear::Y2022, 40_000, 8_177),
        (TaxYear::Y2022, 100_000, 32_663),
        (TaxYear::Y2022, 250_000, 95_663),
        (TaxYear::Y2022, 300_000, 117_328),
        (TaxYear::Y2023, 20_000, 1_956),
        (TaxYear::Y2023, 40_000, 7_828),
        (TaxYear::Y2023, 100_000, 32_027),
        (TaxYear::Y2023, 250_000, 95_027),
        (TaxYear::Y2023, 300_000, 116_692),
        (TaxYear::Y2024, 20_000, 1_725),
        (TaxYear::Y2024, 40_000, 7_461),
        (TaxYear::Y2024, 100_000, 31_363),
        (TaxYear::Y2024, 250_000, 94_363),
        (TaxYear::Y2024, 300_000, 116_028),
        (TaxYear::Y2025, 20_000, 1_639),
        (TaxYear::Y2025, 40_000, 7_320),
        (TaxYear::Y2025, 100_000, 31_088),
        (TaxYear::Y2025, 250_000, 94_088),
        (TaxYear::Y2025, 300_000, 115_753),
        (TaxYear::Y2026, 20_000, 1_570),
        (TaxYear::Y2026, 40_000, 7_209),
        (TaxYear::Y2026, 100_000, 30_864),
        (TaxYear::Y2026, 250_000, 93_864),
        (TaxYear::Y2026, 300_000, 115_529),
    ];

    // Each listed year's tariff matches the independent calculation at five sample incomes.
    #[test]
    fn tariff_matches_independent_calculation_for_every_year() {
        for (year, euros, expected) in ORACLE {
            assert_eq!(
                tax_eur(year, &single(euros * 100)),
                expected,
                "{year:?} at {euros} €"
            );
        }
    }

    // Single amounts from 0 to 300 000 € never give a lower tax at a higher income, in any year.
    proptest! {
        #[test]
        fn tax_never_decreases_as_income_rises(
            index in 0..TaxYear::ALL.len(),
            euros in 0..300_000i128,
        ) {
            let year = TaxYear::ALL[index];
            prop_assert!(tax_eur(year, &single((euros + 1) * 100)) >= tax_eur(year, &single(euros * 100)));
        }

        // One more euro of income adds 0 € or 1 € of tax, since no zone has a rate of 100 % or more.
        #[test]
        fn each_euro_of_income_adds_at_most_one_euro_of_tax(
            index in 0..TaxYear::ALL.len(),
            euros in 0..300_000i128,
        ) {
            let year = TaxYear::ALL[index];
            let step = tax_eur(year, &single((euros + 1) * 100)) - tax_eur(year, &single(euros * 100));
            prop_assert!(step == 0 || step == 1);
        }

        // Joint tax is exactly twice the single tax on the half income, in every year.
        #[test]
        fn joint_tax_is_twice_the_single_tax_on_half_the_income(
            index in 0..TaxYear::ALL.len(),
            euros in 0..300_000i128,
        ) {
            let year = TaxYear::ALL[index];
            prop_assert_eq!(
                tax_eur(year, &joint(euros * 100)),
                2 * tax_eur(year, &single((euros / 2) * 100))
            );
        }

        // Any single amount in the zone, with any cents part, is tax-free in every year.
        #[test]
        fn single_zone_is_tax_free_for_any_cents(
            index in 0..TaxYear::ALL.len(),
            cents in 0..100i128,
        ) {
            let year = TaxYear::ALL[index];
            let allowance = basic_allowance_eur(year);
            let result = assess(year, &single(allowance * 100 + cents)).unwrap();
            prop_assert_eq!(result.income_tax, Cents::new(0));
            prop_assert_eq!(result.taxable_income, Cents::from_euros(allowance));
        }

        // Any i128 cents value, single or joint, in any year returns Ok or Err but never panics.
        #[test]
        fn assess_never_panics(
            index in 0..TaxYear::ALL.len(),
            cents in any::<i128>(),
            joint_filing in any::<bool>(),
        ) {
            let filing = if joint_filing { FilingStatus::Joint } else { FilingStatus::Single };
            let unit = FilingUnit { filing, taxable_income: Cents::new(cents) };
            let _ = assess(TaxYear::ALL[index], &unit);
        }
    }

    proptest! {
        // explain never panics and agrees with assess for random inputs.
        #[test]
        fn explain_never_panics(
            index in 0..TaxYear::ALL.len(),
            cents in any::<i128>(),
            joint_filing in any::<bool>(),
        ) {
            let filing = if joint_filing { FilingStatus::Joint } else { FilingStatus::Single };
            let unit = FilingUnit { filing, taxable_income: Cents::new(cents) };
            if let Ok(e) = explain(TaxYear::ALL[index], &unit) {
                let a = assess(TaxYear::ALL[index], &unit).unwrap();
                prop_assert_eq!(Cents::from_euros(e.income_tax_eur), a.income_tax);
            }
        }

        // tariff_zones numbers are 2..=5 and sorted by from_eur.
        #[test]
        fn tariff_zones_are_well_formed(index in 0..TaxYear::ALL.len()) {
            let zones = tariff_zones(TaxYear::ALL[index]);
            prop_assert!(!zones.is_empty());
            for (i, z) in zones.iter().enumerate() {
                prop_assert_eq!(z.number, (i + 2) as u8);
            }
            for w in zones.windows(2) {
                prop_assert!(w[0].from_eur < w[1].from_eur);
            }
        }

        // basic allowance is non-negative for every year.
        #[test]
        fn basic_allowance_non_negative(index in 0..TaxYear::ALL.len()) {
            prop_assert!(basic_allowance_eur(TaxYear::ALL[index]) >= 0);
        }

        // marginal_rate is finite (not NaN/inf) for in-zone incomes.
        #[test]
        fn marginal_rate_is_finite(index in 0..TaxYear::ALL.len(), euros in 0..300_000i128) {
            let unit = FilingUnit {
                filing: FilingStatus::Single,
                taxable_income: Cents::from_euros(euros),
            };
            let e = explain(TaxYear::ALL[index], &unit).unwrap();
            prop_assert!(e.marginal_rate.is_finite());
            prop_assert!(e.average_rate.is_finite() || euros == 0);
        }
    }
}
