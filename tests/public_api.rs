// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess};

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

// Single filer at 12 348 € pays no income tax and keeps the amount as taxable income.
#[test]
fn single_at_basic_allowance_pays_no_tax() {
    let assessment = assess(TaxYear::Y2026, &single(1_234_800)).unwrap();
    assert_eq!(assessment.income_tax.get(), 0);
    assert_eq!(assessment.taxable_income.get(), 1_234_800);
}

// Joint filer at 24 697 € pays no income tax, because the split half stays in the zone.
#[test]
fn joint_at_split_zone_limit_pays_no_tax() {
    let assessment = assess(TaxYear::Y2026, &joint(2_469_700)).unwrap();
    assert_eq!(assessment.income_tax.get(), 0);
}

// Single filer one euro above the allowance gets the outside-range error, not a tax figure.
#[test]
fn single_above_basic_allowance_returns_outside_range() {
    let error = assess(TaxYear::Y2026, &single(1_234_900)).unwrap_err();
    assert_eq!(
        error,
        Error::OutsideImplementedRange {
            taxable_income_eur: 12_349
        }
    );
}

// Negative taxable income is an input error, not a zero tax.
#[test]
fn negative_income_returns_negative_error() {
    assert_eq!(
        assess(TaxYear::Y2026, &single(-100)).unwrap_err(),
        Error::NegativeTaxableIncome
    );
}

// The negative-income error message names the problem for people reading the output.
#[test]
fn negative_income_error_has_readable_message() {
    assert_eq!(
        Error::NegativeTaxableIncome.to_string(),
        "taxable income is negative"
    );
}

// The out-of-range error message includes the whole-euro amount that caused it.
#[test]
fn outside_range_error_message_names_the_euro_amount() {
    let error = Error::OutsideImplementedRange {
        taxable_income_eur: 12_349,
    };
    assert_eq!(
        error.to_string(),
        "taxable income 12349 EUR is above the implemented tariff range"
    );
}

// Errors can be used as the standard library error trait object.
#[test]
fn error_implements_std_error() {
    let error: Box<dyn std::error::Error> = Box::new(Error::NegativeTaxableIncome);
    assert_eq!(error.to_string(), "taxable income is negative");
}

// Whole euros convert to cents by multiplying by 100.
#[test]
fn from_euros_converts_whole_euros_to_cents() {
    assert_eq!(Cents::from_euros(12_348).get(), 1_234_800);
}

// Cents passed to new are stored as given and returned unchanged by get.
#[test]
fn new_keeps_the_cents_value() {
    assert_eq!(Cents::new(1_234_899).get(), 1_234_899);
}

// Whole-euro rounding drops the cents part of a positive amount.
#[test]
fn whole_euros_drops_cents_of_positive_amounts() {
    assert_eq!(Cents::new(1_234_899).whole_euros(), 12_348);
}

// Whole-euro rounding goes towards negative infinity, so -0.01 € becomes -1 €.
#[test]
fn whole_euros_rounds_negative_amounts_down() {
    assert_eq!(Cents::new(-1).whole_euros(), -1);
    assert_eq!(Cents::new(-100).whole_euros(), -1);
    assert_eq!(Cents::new(-101).whole_euros(), -2);
}

// Cents compare by amount, so the ordering a caller sees matches the money value.
#[test]
fn cents_order_by_amount() {
    assert!(Cents::new(99) < Cents::from_euros(1));
    assert!(Cents::from_euros(-1) < Cents::new(0));
}

// The years fiskus claims to support are exactly 2006 to 2026, and each one exists as a `TaxYear`.
#[test]
fn claimed_years_exist() {
    let claimed: Vec<u16> = (2005..=2026).collect();
    assert_eq!(TaxYear::ALL.len(), claimed.len());
    for (year, variant) in claimed.iter().zip(TaxYear::ALL.iter()) {
        assert_eq!(variant.year(), *year);
        assert_eq!(TaxYear::from_year(*year), Some(*variant));
    }
}

// Years outside the supported range are not representable: `from_year` returns `None`.
#[test]
fn out_of_range_years_are_not_supported() {
    for year in [0u16, 1900, 2004, 2027, 3000, u16::MAX] {
        assert_eq!(TaxYear::from_year(year), None, "{year}");
    }
}
