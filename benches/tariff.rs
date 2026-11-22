// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fiskus::{Cents, FilingStatus, FilingUnit, TaxYear, assess, explain};

// Representative incomes spanning every zone of the tariff, for both filings and all years.
const INCOMES_EUR: &[i128] = &[0, 12_348, 15_000, 30_000, 70_000, 100_000, 277_826, 300_000];

fn assess_all_tiers_years(c: &mut Criterion) {
    c.bench_function("assess_all_tiers_years", |b| {
        b.iter(|| {
            let mut sink = 0i128;
            for year in TaxYear::ALL {
                for &income in INCOMES_EUR {
                    for filing in [FilingStatus::Single, FilingStatus::Joint] {
                        let unit = FilingUnit {
                            filing,
                            taxable_income: Cents::from_euros(income),
                        };
                        let a = assess(black_box(year), black_box(&unit)).unwrap();
                        sink ^= a.income_tax.get();
                    }
                }
            }
            black_box(sink)
        })
    });
}

fn explain_all_tiers_years(c: &mut Criterion) {
    c.bench_function("explain_all_tiers_years", |b| {
        b.iter(|| {
            let mut sink = 0i128;
            for year in TaxYear::ALL {
                for &income in INCOMES_EUR {
                    for filing in [FilingStatus::Single, FilingStatus::Joint] {
                        let unit = FilingUnit {
                            filing,
                            taxable_income: Cents::from_euros(income),
                        };
                        let e = explain(black_box(year), black_box(&unit)).unwrap();
                        sink ^= e.income_tax_eur;
                    }
                }
            }
            black_box(sink)
        })
    });
}

criterion_group!(benches, assess_all_tiers_years, explain_all_tiers_years);
criterion_main!(benches);
