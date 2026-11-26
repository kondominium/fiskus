// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Conformance with the BMF income tax calculator, for every tax year. The layer is described in
//! `docs/internal/testing/README.md`. Run with `make test-conformance`.

use fiskus::{Cents, FilingStatus, FilingUnit, TaxYear, assess};
use std::collections::{BTreeSet, HashMap};

// Every tax year needs at least this many cases.
const MIN_CASES_PER_YEAR: usize = 300;

const HEADER: &str =
    "case_id,year_option,filing_status,taxable_income_cents,income_tax_cents,role,run_id";
const YEAR_OPTIONS: &str = include_str!("../../scripts/conformance/year-options.csv");
const RUNS: &str = include_str!("../fixtures/conformance/runs.csv");

// Open problems: cases where the calculator and fiskus differ. The exact tax is a whole euro, and the
// calculator's floating point gives one euro less. Each case is counted apart and must keep both values,
// so a change to either one fails the test. Each tuple is (case, BMF cents, fiskus cents).
const OPEN_PROBLEMS: [(&str, i128, i128); 2] = [
    ("2019-0215", 10_183_800, 10_184_000),
    ("2020-0335", 10_302_700, 10_302_800),
];

// One fixture per year option, written by scripts/conformance/fetch.sh.
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
    role: String,
    run_id: String,
}

fn filing_status(text: &str) -> FilingStatus {
    match text {
        "single" => FilingStatus::Single,
        "joint" => FilingStatus::Joint,
        other => panic!("unknown filing status {other}"),
    }
}

fn filing_name(filing: FilingStatus) -> &'static str {
    match filing {
        FilingStatus::Single => "single",
        FilingStatus::Joint => "joint",
    }
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
                filing: filing_status(fields[2]),
                taxable_income: fields[3].parse().expect("taxable_income_cents"),
                income_tax: fields[4].parse().expect("income_tax_cents"),
                role: fields[5].to_owned(),
                run_id: fields[6].to_owned(),
            }
        })
        .collect()
}

// Year options and their years, from year-options.csv.
fn year_options() -> Vec<(&'static str, u16, u16)> {
    YEAR_OPTIONS
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let [name, _label, first, last] = fields.as_slice() else {
                return None;
            };
            Some((*name, first.parse().ok()?, last.parse().ok()?))
        })
        .collect()
}

// The tax years an option covers that fiskus implements.
fn years_of(option: &str) -> Vec<TaxYear> {
    let (_, first, last) = year_options()
        .into_iter()
        .find(|(name, _, _)| *name == option)
        .unwrap_or_else(|| panic!("fixture {option} is not in year-options.csv"));
    (first..=last).filter_map(TaxYear::from_year).collect()
}

// CONFORMANCE_YEARS: option names, or all (the default).
fn selected(option: &str) -> bool {
    match std::env::var("CONFORMANCE_YEARS").as_deref() {
        Ok(list) if !list.trim().is_empty() && list.trim() != "all" => {
            list.split(',').any(|item| item.trim() == option)
        }
        _ => true,
    }
}

// The cases of one tax year, counted.
struct Tally {
    cases: usize,
    passed: usize,
    open: usize,
    failures: Vec<String>,
}

fn check(year: TaxYear, csv: &str) -> Tally {
    let mut tally = Tally {
        cases: 0,
        passed: 0,
        open: 0,
        failures: Vec::new(),
    };
    for case in parse_cases(csv) {
        tally.cases += 1;
        let unit = FilingUnit {
            filing: case.filing,
            taxable_income: Cents::new(case.taxable_income),
        };
        let label = format!("{} (tax year {})", case.case_id, year.year());
        let assessed = match assess(year, &unit) {
            Ok(assessment) => assessment,
            Err(error) => {
                tally.failures.push(format!("{label}: {error}"));
                continue;
            }
        };
        let open = OPEN_PROBLEMS
            .iter()
            .find(|(case_id, _, _)| *case_id == case.case_id);
        match (assessed.income_tax == Cents::new(case.income_tax), open) {
            (true, None) => tally.passed += 1,
            (true, Some(_)) => tally.failures.push(format!(
                "{label}: listed as an open problem, but fiskus matches the calculator; remove it from OPEN_PROBLEMS"
            )),
            (false, Some((_, bmf_cents, fiskus_cents)))
                if case.income_tax == *bmf_cents && assessed.income_tax.get() == *fiskus_cents =>
            {
                tally.open += 1;
            }
            (false, Some((_, bmf_cents, fiskus_cents))) => tally.failures.push(format!(
                "{label}: open problem with BMF {bmf_cents} and fiskus {fiskus_cents} cents, but the case gives BMF {} and fiskus {} cents",
                case.income_tax,
                assessed.income_tax.get()
            )),
            (false, None) => tally.failures.push(format!(
                "{label}: fiskus {} cents, expected {} cents",
                assessed.income_tax.get(),
                case.income_tax
            )),
        }
    }
    tally
}

// A share with one decimal, in integers: 9092 of 9500 is 95.7%.
fn percent(part: usize, whole: usize) -> String {
    let tenths = part * 1000 / whole.max(1);
    format!("{}.{}%", tenths / 10, tenths % 10)
}

// One scoreboard row: a tax year and its tally.
struct Row {
    year: u16,
    option: &'static str,
    tally: Tally,
}

fn print_scoreboard(rows: &[Row]) {
    println!(
        "conformance scoreboard (open: the calculator differs from fiskus, listed in OPEN_PROBLEMS)"
    );
    println!(
        "{:<6} {:<7} {:>7} {:>8} {:>6} {:>8} {:>8}",
        "year", "option", "cases", "passed", "open", "failed", "passed"
    );
    let (mut cases, mut passed, mut open, mut failed) = (0, 0, 0, 0);
    for row in rows {
        let failed_here = row.tally.cases - row.tally.passed - row.tally.open;
        println!(
            "{:<6} {:<7} {:>7} {:>8} {:>6} {:>8} {:>8}",
            row.year,
            row.option,
            row.tally.cases,
            row.tally.passed,
            row.tally.open,
            failed_here,
            percent(row.tally.passed, row.tally.cases)
        );
        cases += row.tally.cases;
        passed += row.tally.passed;
        open += row.tally.open;
        failed += failed_here;
    }
    println!(
        "{:<6} {:<7} {:>7} {:>8} {:>6} {:>8} {:>8}",
        "total",
        "",
        cases,
        passed,
        open,
        failed,
        percent(passed, cases)
    );
}

// One test for every selected year option, printed in year order.
#[test]
fn conformance_to_the_bmf_calculator() {
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    for (option, csv) in FIXTURES {
        if !selected(option) {
            continue;
        }
        for year in years_of(option) {
            let tally = check(year, csv);
            if tally.cases < MIN_CASES_PER_YEAR {
                failures.push(format!(
                    "{} (option {option}): {} cases, at least {MIN_CASES_PER_YEAR} are required",
                    year.year(),
                    tally.cases
                ));
            }
            if !tally.failures.is_empty() {
                let first: Vec<&str> = tally.failures.iter().take(20).map(String::as_str).collect();
                failures.push(format!(
                    "{} (option {option}): {} of {} cases fail. First ones:\n{}",
                    year.year(),
                    tally.failures.len(),
                    tally.cases,
                    first.join("\n")
                ));
            }
            rows.push(Row {
                year: year.year(),
                option,
                tally,
            });
        }
    }
    rows.sort_by_key(|row| row.year);
    print_scoreboard(&rows);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

// Layout of every fixture row, and that its run is listed.
#[test]
fn fixture_rows_are_well_formed() {
    let runs: BTreeSet<&str> = RUNS
        .lines()
        .skip(1)
        .filter_map(|line| line.split(',').next())
        .collect();
    for (option, csv) in FIXTURES {
        assert_eq!(csv.lines().next(), Some(HEADER), "{option}: header");
        for (index, case) in parse_cases(csv).iter().enumerate() {
            let expected_id = format!("{option}-{:04}", index + 1);
            assert_eq!(
                case.case_id, expected_id,
                "{option}: case ids are sequential"
            );
            assert_eq!(case.year_option, option, "{option}: {}", case.case_id);
            assert!(!case.role.is_empty(), "{}: role", case.case_id);
            assert!(
                runs.contains(case.run_id.as_str()),
                "{}: run {} is not in runs.csv",
                case.case_id,
                case.run_id
            );
            assert!(
                case.taxable_income >= 0,
                "{}: negative income",
                case.case_id
            );
            assert!(case.income_tax >= 0, "{}: negative tax", case.case_id);
        }
    }
}

// No income is checked twice for one filing status.
#[test]
fn fixture_cases_are_unique() {
    for (option, csv) in FIXTURES {
        let mut seen = BTreeSet::new();
        for case in parse_cases(csv) {
            assert!(
                seen.insert((filing_name(case.filing), case.taxable_income)),
                "{option}: {} repeats an income",
                case.case_id
            );
        }
    }
}

// Tax never falls as income rises.
#[test]
fn calculator_tax_never_decreases_with_income() {
    for (option, csv) in FIXTURES {
        for filing in [FilingStatus::Single, FilingStatus::Joint] {
            let mut rows: Vec<(i128, i128, String)> = parse_cases(csv)
                .into_iter()
                .filter(|case| case.filing == filing)
                .map(|case| (case.taxable_income, case.income_tax, case.case_id))
                .collect();
            rows.sort_unstable();
            for pair in rows.windows(2) {
                assert!(
                    pair[0].1 <= pair[1].1,
                    "{option}: {} pays more than {} with less income",
                    pair[0].2,
                    pair[1].2
                );
            }
        }
    }
}

// Joint tax is twice the single tax on half, where the calculator has both.
#[test]
fn calculator_joint_tax_is_twice_the_single_tax_on_half() {
    for (option, csv) in FIXTURES {
        let cases = parse_cases(csv);
        let single: HashMap<i128, i128> = cases
            .iter()
            .filter(|case| case.filing == FilingStatus::Single)
            .map(|case| (case.taxable_income, case.income_tax))
            .collect();
        for case in cases
            .iter()
            .filter(|case| case.filing == FilingStatus::Joint)
        {
            if case.taxable_income % 200 != 0 {
                continue;
            }
            if let Some(half_tax) = single.get(&(case.taxable_income / 2)) {
                assert_eq!(
                    case.income_tax,
                    2 * half_tax,
                    "{option}: {} is not twice the single tax on half",
                    case.case_id
                );
            }
        }
    }
}

// Every tax year has a fixture.
#[test]
fn every_tax_year_has_a_fixture() {
    let covered: BTreeSet<u16> = FIXTURES
        .iter()
        .flat_map(|(option, _)| years_of(option))
        .map(TaxYear::year)
        .collect();
    for year in TaxYear::ALL {
        assert!(
            covered.contains(&year.year()),
            "tax year {} has no fixture",
            year.year()
        );
    }
}

// Every open problem is a fixture case, with the BMF value that its entry records.
#[test]
fn open_problems_are_fixture_cases() {
    for (case_id, bmf_cents, _) in OPEN_PROBLEMS {
        let found = FIXTURES.iter().any(|(_, csv)| {
            parse_cases(csv)
                .iter()
                .any(|case| case.case_id == case_id && case.income_tax == bmf_cents)
        });
        assert!(
            found,
            "open problem {case_id} is not a fixture case with BMF {bmf_cents} cents"
        );
    }
}
