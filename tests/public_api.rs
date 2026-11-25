// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Black-box tests of the public API. They use only what a caller can import, and no fixed years
//! or amounts: every expected value comes from the API itself or from `TaxYear::ALL`.

use fiskus::{
    Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur, explain,
    solidarity_free_threshold_eur, solidarity_source, solidarity_surcharge, tariff_zones,
};

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

#[test]
fn every_listed_year_pays_no_tax_at_its_basic_allowance() {
    for year in TaxYear::ALL {
        let allowance = basic_allowance_eur(year);
        let assessment = assess(year, &single(allowance * 100)).unwrap();
        assert_eq!(assessment.income_tax, Cents::new(0), "{year:?}");
        assert_eq!(
            assessment.taxable_income,
            Cents::from_euros(allowance),
            "{year:?}"
        );
    }
}

#[test]
fn every_year_with_zones_taxes_income_above_the_allowance() {
    for year in TaxYear::ALL
        .into_iter()
        .filter(|y| !tariff_zones(*y).is_empty())
    {
        let income = basic_allowance_eur(year) * 2;
        let assessment = assess(year, &single(income * 100)).unwrap();
        assert!(assessment.income_tax.get() > 0, "{year:?}");
        assert!(assessment.income_tax.whole_euros() < income, "{year:?}");
    }
}

#[test]
fn joint_tax_is_twice_the_tax_on_half_the_income() {
    for year in TaxYear::ALL
        .into_iter()
        .filter(|y| !tariff_zones(*y).is_empty())
    {
        let income_eur = basic_allowance_eur(year) * 3;
        let joint_tax = assess(year, &joint(income_eur * 100)).unwrap().income_tax;
        let half_tax = assess(year, &single(income_eur / 2 * 100))
            .unwrap()
            .income_tax;
        assert_eq!(
            joint_tax,
            Cents::from_euros(2 * half_tax.whole_euros()),
            "{year:?}"
        );
    }
}

#[test]
fn explanation_and_assessment_agree() {
    for year in TaxYear::ALL
        .into_iter()
        .filter(|y| !tariff_zones(*y).is_empty())
    {
        let unit = single(basic_allowance_eur(year) * 2 * 100);
        let explanation = explain(year, &unit).unwrap();
        let assessment = assess(year, &unit).unwrap();
        assert_eq!(
            Cents::from_euros(explanation.income_tax_eur),
            assessment.income_tax,
            "{year:?}"
        );
        assert_eq!(explanation.year, year);
    }
}

#[test]
fn explanation_names_the_source_only_for_years_with_a_tariff() {
    for year in TaxYear::ALL {
        let explanation = explain(year, &single(0)).unwrap();
        assert_eq!(
            explanation.source.is_some(),
            !tariff_zones(year).is_empty(),
            "{year:?}"
        );
    }
}

#[test]
fn zone_numbers_run_from_two_upward_in_order() {
    for year in TaxYear::ALL {
        for (index, zone) in tariff_zones(year).iter().enumerate() {
            assert_eq!(usize::from(zone.number), index + 2, "{year:?}");
        }
    }
}

#[test]
fn negative_income_is_an_error_in_every_year() {
    for year in TaxYear::ALL {
        assert_eq!(
            assess(year, &single(-1)).unwrap_err(),
            Error::NegativeTaxableIncome,
            "{year:?}"
        );
        assert_eq!(
            explain(year, &joint(-100)).unwrap_err(),
            Error::NegativeTaxableIncome
        );
    }
}

#[test]
fn year_without_zones_reports_out_of_range_above_the_allowance() {
    for year in TaxYear::ALL
        .into_iter()
        .filter(|y| tariff_zones(*y).is_empty())
    {
        let above = (basic_allowance_eur(year) + 1) * 100;
        assert_eq!(
            assess(year, &single(above)).unwrap_err(),
            Error::OutsideImplementedRange {
                taxable_income_eur: basic_allowance_eur(year) + 1
            },
            "{year:?}"
        );
    }
}

#[test]
fn error_messages_are_readable() {
    assert_eq!(
        Error::NegativeTaxableIncome.to_string(),
        "taxable income is negative"
    );
    let error = Error::OutsideImplementedRange {
        taxable_income_eur: 12_349,
    };
    assert_eq!(
        error.to_string(),
        "taxable income 12349 EUR is above the implemented tariff range"
    );
}

#[test]
fn error_implements_the_standard_error_trait() {
    let error: Box<dyn std::error::Error> = Box::new(Error::NegativeTaxableIncome);
    assert_eq!(error.to_string(), "taxable income is negative");
}

#[test]
fn solidarity_surcharge_is_zero_at_the_threshold_of_each_year() {
    for year in TaxYear::ALL {
        for filing in [FilingStatus::Single, FilingStatus::Joint] {
            let Some(threshold) = solidarity_free_threshold_eur(year, filing) else {
                continue;
            };
            let at_threshold = solidarity_surcharge(year, filing, Cents::from_euros(threshold));
            assert_eq!(at_threshold, Some(Cents::new(0)), "{year:?} {filing:?}");
        }
    }
}

#[test]
fn solidarity_surcharge_is_none_for_a_year_without_a_threshold() {
    for year in TaxYear::ALL {
        let has_threshold = solidarity_free_threshold_eur(year, FilingStatus::Single).is_some();
        let surcharge =
            solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(1_000_000));
        assert_eq!(surcharge.is_some(), has_threshold, "{year:?}");
        assert_eq!(solidarity_source(year).is_some(), has_threshold, "{year:?}");
    }
}

#[test]
fn solidarity_surcharge_applies_above_the_threshold_of_each_year() {
    for year in TaxYear::ALL {
        let Some(threshold) = solidarity_free_threshold_eur(year, FilingStatus::Single) else {
            continue;
        };
        let above =
            solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(threshold + 1));
        assert!(
            above.is_some_and(|surcharge| surcharge.get() > 0),
            "{year:?}"
        );
    }
}

#[test]
fn joint_threshold_is_at_least_the_single_one() {
    for year in TaxYear::ALL {
        let single = solidarity_free_threshold_eur(year, FilingStatus::Single);
        let joint = solidarity_free_threshold_eur(year, FilingStatus::Joint);
        assert_eq!(single.is_some(), joint.is_some(), "{year:?}");
        if let (Some(single), Some(joint)) = (single, joint) {
            assert!(joint >= single, "{year:?}");
        }
    }
}

#[test]
fn cents_convert_to_and_from_euros() {
    let euros = 1_234;
    assert_eq!(Cents::from_euros(euros).get(), euros * 100);
    assert_eq!(Cents::new(euros * 100 + 99).whole_euros(), euros);
    assert_eq!(Cents::new(-1).whole_euros(), -1);
}

#[test]
fn cents_order_by_amount() {
    assert!(Cents::new(99) < Cents::from_euros(1));
    assert!(Cents::from_euros(-1) < Cents::new(0));
}

#[test]
fn listed_years_are_contiguous_and_round_trip() {
    for pair in TaxYear::ALL.windows(2) {
        assert_eq!(pair[0].year() + 1, pair[1].year());
    }
    for year in TaxYear::ALL {
        assert_eq!(TaxYear::from_year(year.year()), Some(year));
    }
}

#[test]
fn years_outside_the_listed_range_are_not_supported() {
    let first = TaxYear::ALL[0].year();
    let last = TaxYear::ALL[TaxYear::ALL.len() - 1].year();
    for year in [0, first - 1, last + 1, u16::MAX] {
        assert_eq!(TaxYear::from_year(year), None, "{year}");
    }
}
