// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use fiskus::{Cents, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur};

fn assess_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("assess");

    group.throughput(Throughput::Elements((TaxYear::ALL.len() * 4) as u64));
    group.bench_function("in_zone", |b| {
        b.iter(|| {
            let mut sink = 0i128;
            for year in TaxYear::ALL {
                let allowance = basic_allowance_eur(year);
                for &(filing, euros) in &[
                    (FilingStatus::Single, 0),
                    (FilingStatus::Single, allowance),
                    (FilingStatus::Joint, 0),
                    (FilingStatus::Joint, allowance * 2 + 1),
                ] {
                    let unit = FilingUnit {
                        filing,
                        taxable_income: Cents::from_euros(euros),
                    };
                    let assessment = assess(black_box(year), black_box(&unit)).unwrap();
                    sink ^= assessment.income_tax.get();
                }
            }
            black_box(sink)
        })
    });

    group.throughput(Throughput::Elements((TaxYear::ALL.len() * 3) as u64));
    group.bench_function("outside_zone", |b| {
        b.iter(|| {
            let mut errors = 0usize;
            for year in TaxYear::ALL {
                let allowance = basic_allowance_eur(year);
                for &(filing, euros) in &[
                    (FilingStatus::Single, allowance + 1),
                    (FilingStatus::Joint, allowance * 2 + 2),
                    (FilingStatus::Single, -1),
                ] {
                    let unit = FilingUnit {
                        filing,
                        taxable_income: Cents::from_euros(euros),
                    };
                    errors += usize::from(assess(black_box(year), black_box(&unit)).is_err());
                }
            }
            black_box(errors)
        })
    });

    group.finish();
}

criterion_group!(benches, assess_calls);
criterion_main!(benches);
