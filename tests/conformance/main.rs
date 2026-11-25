// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//
// Conformance: fiskus against recorded results from an official calculator. The fixtures are generated
// by the scripts described in docs/internal/testing/conformance.md. The tests are gated behind the
// `conformance` feature, so `make gates` stays green. Run them with `make test-conformance`, choose years
// with YEARS=2010,2026 or YEARS=all, and read the scoreboard it prints.
use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess};
use std::io::Write;

// Years run when CONFORMANCE_YEARS is unset. 2026 is left out for now.
const DEFAULT_LAST_YEAR: u32 = 2025;

// Every year needs at least this many cases. A smaller sample says too little about the year, so it fails.
const MIN_CASES_PER_YEAR: usize = 300;

struct Case {
    case_id: String,
    year_option: String,
    filing: FilingStatus,
    taxable_income: i128,
    income_tax: i128,
}

fn parse_cases(csv: &str) -> Vec<Case> {
    csv.lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            assert_eq!(fields.len(), 7, "malformed fixture row: {line}");
            Case {
                case_id: fields[0].to_owned(),
                year_option: fields[1].to_owned(),
                filing: match fields[2] {
                    "single" => FilingStatus::Single,
                    "joint" => FilingStatus::Joint,
                    other => panic!("unknown filing status {other}"),
                },
                taxable_income: fields[3].parse().expect("taxable_income_cents"),
                income_tax: fields[4].parse().expect("income_tax_cents"),
            }
        })
        .collect()
}

fn tax_year(year_option: &str) -> Option<TaxYear> {
    // The year_option is the BMF dropdown value, which may cover several years.
    // fiskus has one TaxYear per calendar year, so pick a year that falls inside
    // the group and that fiskus implements. The BMF uses the same tariff for
    // every year in the group, so any year in the range gives the same answer.
    match year_option {
        "2026" => Some(TaxYear::Y2026),
        "2025" => Some(TaxYear::Y2025),
        "2024" => Some(TaxYear::Y2024),
        "2023" => Some(TaxYear::Y2023),
        "2022" => Some(TaxYear::Y2022),
        "2021" => Some(TaxYear::Y2021),
        "2020" => Some(TaxYear::Y2020),
        "2019" => Some(TaxYear::Y2019),
        "2018" => Some(TaxYear::Y2018),
        "2017" => Some(TaxYear::Y2017),
        "2016" => Some(TaxYear::Y2016),
        "2015" => Some(TaxYear::Y2015),
        "2014" => Some(TaxYear::Y2014),
        "2013" => Some(TaxYear::Y2013),
        "2010" => Some(TaxYear::Y2010), // covers 2010-2012
        "2009" => Some(TaxYear::Y2009),
        "2007" => Some(TaxYear::Y2007), // covers 2007-2008
        "2005" => Some(TaxYear::Y2005), // covers 2005-2006
        "2004" => Some(TaxYear::Y2004),
        "2002" => Some(TaxYear::Y2003), // covers 2002-2003
        _ => None,
    }
}

// `all` runs every year. A list such as `2010,2026` runs those years. Unset or empty runs up to DEFAULT_LAST_YEAR.
fn selected(year: &str) -> bool {
    match std::env::var("CONFORMANCE_YEARS").as_deref() {
        Ok("all") => true,
        Ok(list) if !list.trim().is_empty() => list.split(',').any(|item| item.trim() == year),
        _ => year
            .parse::<u32>()
            .is_ok_and(|value| value <= DEFAULT_LAST_YEAR),
    }
}

#[derive(Default)]
struct Score {
    passed: usize,
    zone_missing: usize,
    mismatched: usize,
}

// Holds the stderr lock for the whole line, so the years that run concurrently cannot interleave.
fn report(line: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{line}");
}

fn check_year(year: &str, csv: &str) {
    if !selected(year) {
        eprintln!("{year}: not selected, skipped (set CONFORMANCE_YEARS to run it)");
        return;
    }

    let mut score = Score::default();
    let mut failures = Vec::new();
    for case in parse_cases(csv) {
        let unit = FilingUnit {
            filing: case.filing,
            taxable_income: Cents::new(case.taxable_income),
        };
        match tax_year(&case.year_option).map(|tax_year| assess(tax_year, &unit)) {
            None => {
                score.zone_missing += 1;
                failures.push(format!(
                    "{}: tax year {} is not in fiskus yet",
                    case.case_id, case.year_option
                ));
            }
            Some(Ok(assessment)) if assessment.income_tax == Cents::new(case.income_tax) => {
                score.passed += 1;
            }
            Some(Ok(assessment)) => {
                score.mismatched += 1;
                failures.push(format!(
                    "{}: fiskus {} cents, expected {} cents",
                    case.case_id,
                    assessment.income_tax.get(),
                    case.income_tax
                ));
            }
            Some(Err(Error::OutsideImplementedRange { .. })) => {
                score.zone_missing += 1;
                failures.push(format!("{}: tariff zone not in fiskus yet", case.case_id));
            }
            Some(Err(error)) => {
                score.mismatched += 1;
                failures.push(format!("{}: {error}", case.case_id));
            }
        }
    }

    let cases = score.passed + score.zone_missing + score.mismatched;
    report(&format!(
        "score year={year} cases={cases} passed={} zone_missing={} mismatched={}",
        score.passed, score.zone_missing, score.mismatched
    ));
    assert!(
        cases >= MIN_CASES_PER_YEAR,
        "{year}: {cases} cases, at least {MIN_CASES_PER_YEAR} are required"
    );
    let first: Vec<&str> = failures.iter().take(20).map(String::as_str).collect();
    assert!(
        failures.is_empty(),
        "{year}: {} of {cases} cases fail. First ones:\n{}",
        failures.len(),
        first.join("\n")
    );
}

macro_rules! year_tests {
    ($($name:ident: $year:literal => $csv:literal,)*) => {
        $(
            #[test]
            fn $name() {
                check_year($year, include_str!($csv));
            }
        )*
    };
}

// One test per euro year. libtest runs them concurrently, and CONFORMANCE_YEARS picks which ones run.
year_tests! {
    year_2026: "2026" => "../fixtures/conformance/2026.csv",
    year_2025: "2025" => "../fixtures/conformance/2025.csv",
    year_2024: "2024" => "../fixtures/conformance/2024.csv",
    year_2023: "2023" => "../fixtures/conformance/2023.csv",
    year_2022: "2022" => "../fixtures/conformance/2022.csv",
    year_2021: "2021" => "../fixtures/conformance/2021.csv",
    year_2020: "2020" => "../fixtures/conformance/2020.csv",
    year_2019: "2019" => "../fixtures/conformance/2019.csv",
    year_2018: "2018" => "../fixtures/conformance/2018.csv",
    year_2017: "2017" => "../fixtures/conformance/2017.csv",
    year_2016: "2016" => "../fixtures/conformance/2016.csv",
    year_2015: "2015" => "../fixtures/conformance/2015.csv",
    year_2014: "2014" => "../fixtures/conformance/2014.csv",
    year_2013: "2013" => "../fixtures/conformance/2013.csv",
    year_2010: "2010" => "../fixtures/conformance/2010.csv",
    year_2009: "2009" => "../fixtures/conformance/2009.csv",
    year_2007: "2007" => "../fixtures/conformance/2007.csv",
    year_2005: "2005" => "../fixtures/conformance/2005.csv",
    year_2004: "2004" => "../fixtures/conformance/2004.csv",
    year_2002: "2002" => "../fixtures/conformance/2002.csv",
}
