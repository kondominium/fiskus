// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use fiskus::{Cents, FilingStatus, FilingUnit, TaxYear, assess};

// Inputs inside the implemented zone: the basic allowance for single, the split limit for joint.
const IN_ZONE: &[(FilingStatus, i128)] = &[
    (FilingStatus::Single, 0),
    (FilingStatus::Single, 12_348),
    (FilingStatus::Joint, 0),
    (FilingStatus::Joint, 24_697),
];

// Inputs the implemented zone rejects: one euro above each limit, and negative income.
const OUTSIDE_ZONE: &[(FilingStatus, i128)] = &[
    (FilingStatus::Single, 12_349),
    (FilingStatus::Joint, 24_698),
    (FilingStatus::Single, -1),
];

fn assess_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("assess");

    group.throughput(Throughput::Elements(IN_ZONE.len() as u64));
    group.bench_function("in_zone", |b| {
        b.iter(|| {
            let mut sink = 0i128;
            for &(filing, euros) in IN_ZONE {
                let unit = FilingUnit {
                    filing,
                    taxable_income: Cents::from_euros(euros),
                };
                let assessment = assess(black_box(TaxYear::Y2026), black_box(&unit)).unwrap();
                sink ^= assessment.income_tax.get();
            }
            black_box(sink)
        })
    });

    group.throughput(Throughput::Elements(OUTSIDE_ZONE.len() as u64));
    group.bench_function("outside_zone", |b| {
        b.iter(|| {
            let mut errors = 0usize;
            for &(filing, euros) in OUTSIDE_ZONE {
                let unit = FilingUnit {
                    filing,
                    taxable_income: Cents::from_euros(euros),
                };
                errors += usize::from(assess(black_box(TaxYear::Y2026), black_box(&unit)).is_err());
            }
            black_box(errors)
        })
    });

    group.finish();
}

criterion_group!(benches, assess_calls);
criterion_main!(benches);
