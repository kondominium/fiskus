// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Compares fiskus with the income tax results of the BMF calculator, for every euro year option.
//!
//! The fixtures are written by `scripts/conformance/fetch.sh`. The test prints a scoreboard with one row
//! per year option, and fails when any case differs. Run it with `make test-conformance`, choose years
//! with `CONFORMANCE_YEARS=2010,2026` or `all`, and read the scoreboard it prints.

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess};

// Years run when CONFORMANCE_YEARS is unset. 2026 is left out for now.
const DEFAULT_LAST_YEAR: u32 = 2025;

// Every year needs at least this many cases. A smaller sample says too little about the year, so it fails.
const MIN_CASES_PER_YEAR: usize = 300;

// One fixture per euro year option, as written by scripts/conformance/fetch.sh.
const FIXTURES: [(&str, &str); 20] = [
    ("2026", include_str!("../fixtures/conformance/2026.csv")),
    ("2025", include_str!("../fixtures/conformance/2025.csv")),
    ("2024", include_str!("../fixtures/conformance/2024.csv")),
    ("2023", include_str!("../fixtures/conformance/2023.csv")),
    ("2022", include_str!("../fixtures/conformance/2022.csv")),
    ("2021", include_str!("../fixtures/conformance/2021.csv")),
    ("2020", include_str!("../fixtures/conformance/2020.csv")),
    ("2019", include_str!("../fixtures/conformance/2019.csv")),
    ("2018", include_str!("../fixtures/conformance/2018.csv")),
    ("2017", include_str!("../fixtures/conformance/2017.csv")),
    ("2016", include_str!("../fixtures/conformance/2016.csv")),
    ("2015", include_str!("../fixtures/conformance/2015.csv")),
    ("2014", include_str!("../fixtures/conformance/2014.csv")),
    ("2013", include_str!("../fixtures/conformance/2013.csv")),
    ("2010", include_str!("../fixtures/conformance/2010.csv")),
    ("2009", include_str!("../fixtures/conformance/2009.csv")),
    ("2007", include_str!("../fixtures/conformance/2007.csv")),
    ("2005", include_str!("../fixtures/conformance/2005.csv")),
    ("2004", include_str!("../fixtures/conformance/2004.csv")),
    ("2002", include_str!("../fixtures/conformance/2002.csv")),
];

#[expect(
    clippy::struct_field_names,
    reason = "the fields are the columns of a fixture row"
)]
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

// One row of the scoreboard: how the cases of one year option came out.
struct Row {
    year: &'static str,
    cases: usize,
    passed: usize,
    zone_missing: usize,
    mismatched: usize,
}

// Checks every case of one fixture. Returns the row, and a message for each failing case.
fn check_year(year: &'static str, csv: &str) -> (Row, Vec<String>) {
    let mut row = Row {
        year,
        cases: 0,
        passed: 0,
        zone_missing: 0,
        mismatched: 0,
    };
    let mut failures = Vec::new();
    for case in parse_cases(csv) {
        row.cases += 1;
        let unit = FilingUnit {
            filing: case.filing,
            taxable_income: Cents::new(case.taxable_income),
        };
        match tax_year(&case.year_option).map(|tax_year| assess(tax_year, &unit)) {
            None => {
                row.zone_missing += 1;
                failures.push(format!(
                    "{}: tax year {} is not in fiskus yet",
                    case.case_id, case.year_option
                ));
            }
            Some(Ok(assessment)) if assessment.income_tax == Cents::new(case.income_tax) => {
                row.passed += 1;
            }
            Some(Ok(assessment)) => {
                row.mismatched += 1;
                failures.push(format!(
                    "{}: fiskus {} cents, expected {} cents",
                    case.case_id,
                    assessment.income_tax.get(),
                    case.income_tax
                ));
            }
            Some(Err(Error::OutsideImplementedRange { .. })) => {
                row.zone_missing += 1;
                failures.push(format!("{}: tariff zone not in fiskus yet", case.case_id));
            }
            Some(Err(error)) => {
                row.mismatched += 1;
                failures.push(format!("{}: {error}", case.case_id));
            }
        }
    }
    (row, failures)
}

// A share with one decimal, in integer arithmetic: 9092 of 9500 is "95.7%".
fn percent(part: usize, whole: usize) -> String {
    let tenths = part * 1000 / whole.max(1);
    format!("{}.{}%", tenths / 10, tenths % 10)
}

fn print_scoreboard(rows: &[Row]) {
    println!(
        "conformance scoreboard (zone missing: the tariff zone is not in fiskus yet; mismatched: fiskus disagrees with the calculator)"
    );
    println!(
        "{:<6} {:>7} {:>8} {:>8} {:>13} {:>11} {:>8}",
        "year", "cases", "passed", "failed", "zone missing", "mismatched", "passed"
    );
    let mut total = Row {
        year: "total",
        cases: 0,
        passed: 0,
        zone_missing: 0,
        mismatched: 0,
    };
    for row in rows {
        print_row(row);
        total.cases += row.cases;
        total.passed += row.passed;
        total.zone_missing += row.zone_missing;
        total.mismatched += row.mismatched;
    }
    print_row(&total);
}

fn print_row(row: &Row) {
    println!(
        "{:<6} {:>7} {:>8} {:>8} {:>13} {:>11} {:>8}",
        row.year,
        row.cases,
        row.passed,
        row.cases - row.passed,
        row.zone_missing,
        row.mismatched,
        percent(row.passed, row.cases)
    );
}

// One test for every selected year, so the scoreboard is printed whole, in year order.
#[test]
fn conformance_to_the_bmf_calculator() {
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    for (year, csv) in FIXTURES {
        if !selected(year) {
            continue;
        }
        let (row, year_failures) = check_year(year, csv);
        if row.cases < MIN_CASES_PER_YEAR {
            failures.push(format!(
                "{year}: {} cases, at least {MIN_CASES_PER_YEAR} are required",
                row.cases
            ));
        }
        if !year_failures.is_empty() {
            let first: Vec<&str> = year_failures.iter().take(20).map(String::as_str).collect();
            failures.push(format!(
                "{year}: {} of {} cases fail. First ones:\n{}",
                year_failures.len(),
                row.cases,
                first.join("\n")
            ));
        }
        rows.push(row);
    }
    rows.sort_by_key(|row| row.year);
    print_scoreboard(&rows);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
