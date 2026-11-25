// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Writes conformance cases to stdout, one per line: `year_option,filing_status,taxable_income_cents,role`.
//!
//! `kinks` gives the incomes at the basic allowance and at each zone start: one euro below, at, with
//! 0.99 € more, and one euro above, for single and for joint filing. It adds zero, one odd joint income
//! (the halving rounds it down) and two tail incomes. The starts come from `rules/`, so a wrong start shows
//! as a mismatch. `samples` gives `count` seeded incomes per filing status, spread over four bands.
//!
//! Usage: `cargo run -q --example conformance_points -- <year_option|all> <kinks|samples> [count] [seed]`.
//! Pipe the output into `scripts/conformance/fetch.sh`. The default seed is the year option times 10 000
//! plus the cases its fixture already has, so a top-up draws incomes the fixture does not have yet.
#![expect(
    clippy::print_stdout,
    reason = "the generator writes its cases to stdout, where fetch.sh reads them"
)]

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use fiskus::{TaxYear, basic_allowance_eur, tariff_zones};

const YEAR_OPTIONS: &str = include_str!("year-options.csv");

// The euro years: `all` covers the year options whose first year is this one or later.
const FIRST_EURO_YEAR: u16 = 2002;

// Tail incomes in whole euros: one million and ten million.
const TAIL_EUROS: [i128; 2] = [1_000_000, 10_000_000];

// Income bands for samples, in whole euros, as inclusive (low, high) pairs.
const BANDS: [(u64, u64); 4] = [
    (1, 15_000),
    (15_001, 70_000),
    (70_001, 300_000),
    (300_001, 10_000_000),
];

// One case: filing status, taxable income in cents, and why the case is here.
type Case = (&'static str, i128, &'static str);

struct YearOption {
    name: String,
    first_year: u16,
    last_year: u16,
}

fn year_options() -> Vec<YearOption> {
    YEAR_OPTIONS
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let [name, _label, first, last] = fields.as_slice() else {
                return None;
            };
            Some(YearOption {
                name: (*name).to_owned(),
                first_year: first.parse().ok()?,
                last_year: last.parse().ok()?,
            })
        })
        .collect()
}

// The option covers several years, and fiskus has one TaxYear per year. Option 2002 covers 2002 and
// 2003, and fiskus has no 2002, so the first year it implements is used.
fn fiskus_year(option: &YearOption) -> Option<TaxYear> {
    (option.first_year..=option.last_year).find_map(TaxYear::from_year)
}

fn kink_cases(year: TaxYear) -> Vec<Case> {
    let mut starts = vec![basic_allowance_eur(year)];
    starts.extend(tariff_zones(year).iter().map(|zone| zone.from_eur));
    starts.sort_unstable();
    starts.dedup();

    let mut cases: Vec<Case> = vec![("single", 0, "zero"), ("joint", 0, "zero")];
    for start in starts {
        for (filing, multiple) in [("single", 1_i128), ("joint", 2)] {
            let euros = start * multiple;
            cases.push((filing, (euros - 1) * 100, "kink_minus"));
            cases.push((filing, euros * 100, "kink"));
            cases.push((filing, euros * 100 + 99, "kink_cents"));
            cases.push((filing, (euros + 1) * 100, "kink_plus"));
        }
        cases.push(("joint", (2 * start + 1) * 100, "odd"));
    }
    for euros in TAIL_EUROS {
        for filing in ["single", "joint"] {
            cases.push((filing, euros * 100, "tail"));
        }
    }
    // Boundaries can touch: the allowance and the first zone start are one euro apart, so one income
    // can be a kink of two boundaries. The first role wins, and each income is fetched once.
    let mut seen = BTreeSet::new();
    cases.retain(|(filing, cents, _)| seen.insert((*filing, *cents)));
    cases
}

// SplitMix64: a small, fixed generator, so the same seed always gives the same incomes.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn within(&mut self, low: u64, high: u64) -> u64 {
        low + self.next_u64() % (high - low + 1)
    }
}

fn sample_cases(count: usize, seed: u64) -> Vec<Case> {
    let mut rng = SplitMix64(seed);
    let mut cases = Vec::new();
    for filing in ["single", "joint"] {
        for (low, high) in BANDS.iter().cycle().take(count) {
            let euros = rng.within(*low, *high);
            let cents = rng.within(0, 99);
            cases.push((
                filing,
                i128::from(euros) * 100 + i128::from(cents),
                "sample",
            ));
        }
    }
    cases
}

// The cases the option's fixture already has. A new seed starts after them.
fn fixture_rows(option: &str) -> u64 {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/conformance")
        .join(format!("{option}.csv"));
    let rows = fs::read_to_string(path).map_or(0, |text| text.lines().count().saturating_sub(1));
    u64::try_from(rows).unwrap_or(0)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [requested, kind, rest @ ..] = args.as_slice() else {
        return Err(
            "usage: conformance_points <year_option|all> <kinks|samples> [count] [seed]".into(),
        );
    };
    let count: usize = match rest.first() {
        Some(text) => text.parse().map_err(|error| format!("count: {error}"))?,
        None => 50,
    };
    let seed: Option<u64> = match rest.get(1) {
        Some(text) => Some(text.parse().map_err(|error| format!("seed: {error}"))?),
        None => None,
    };

    let options: Vec<YearOption> = year_options()
        .into_iter()
        .filter(|option| {
            if requested == "all" {
                option.first_year >= FIRST_EURO_YEAR
            } else {
                option.name == *requested
            }
        })
        .collect();
    if options.is_empty() {
        return Err(format!("no year option named {requested}"));
    }

    for option in &options {
        let year = fiskus_year(option)
            .ok_or_else(|| format!("{}: no year in the option is in fiskus", option.name))?;
        let cases = match kind.as_str() {
            "kinks" => kink_cases(year),
            "samples" => {
                let option_seed =
                    option.name.parse::<u64>().unwrap_or(0) * 10_000 + fixture_rows(&option.name);
                sample_cases(count, seed.unwrap_or(option_seed))
            }
            other => return Err(format!("unknown kind {other}: use kinks or samples")),
        };
        for (filing, cents, role) in cases {
            println!("{},{filing},{cents},{role}", option.name);
        }
    }
    Ok(())
}
