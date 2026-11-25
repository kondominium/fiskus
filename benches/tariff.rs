// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Benchmarks the calculation paths: each filing status, the zero zone, the zone boundaries of
//! every listed year, the explanation, the surcharge and the error path.
//!
//! The inputs come from `TaxYear::ALL`, the basic allowance and the zone data, so the benchmark
//! follows the rule files without edits.

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use fiskus::{
    Cents, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur, explain,
    solidarity_surcharge, tariff_zones,
};

fn units_for(year: TaxYear) -> Vec<(TaxYear, FilingUnit)> {
    let allowance = basic_allowance_eur(year);
    let zone_starts = tariff_zones(year).into_iter().map(|zone| zone.from_eur);
    let incomes = [0, allowance, allowance * 2].into_iter().chain(zone_starts);
    incomes
        .flat_map(|euros| {
            [FilingStatus::Single, FilingStatus::Joint].map(|filing| {
                (
                    year,
                    FilingUnit {
                        filing,
                        taxable_income: Cents::from_euros(euros),
                    },
                )
            })
        })
        .collect()
}

fn all_units() -> Vec<(TaxYear, FilingUnit)> {
    TaxYear::ALL.into_iter().flat_map(units_for).collect()
}

fn assess_calls(c: &mut Criterion) {
    let units = all_units();
    let mut group = c.benchmark_group("assess");
    group.throughput(Throughput::Elements(units.len() as u64));
    group.bench_function("every_year_and_boundary", |b| {
        b.iter(|| {
            units.iter().fold(0i128, |sink, (year, unit)| {
                sink ^ assess(black_box(*year), black_box(unit)).map_or(0, |a| a.income_tax.get())
            })
        })
    });
    group.finish();
}

fn explain_calls(c: &mut Criterion) {
    let units = all_units();
    let mut group = c.benchmark_group("explain");
    group.throughput(Throughput::Elements(units.len() as u64));
    group.bench_function("every_year_and_boundary", |b| {
        b.iter(|| {
            units.iter().fold(0i128, |sink, (year, unit)| {
                sink ^ explain(black_box(*year), black_box(unit)).map_or(0, |e| e.income_tax_eur)
            })
        })
    });
    group.finish();
}

fn error_path(c: &mut Criterion) {
    let year = *TaxYear::ALL.last().expect("at least one year is listed");
    let negative = FilingUnit {
        filing: FilingStatus::Single,
        taxable_income: Cents::new(-1),
    };
    let mut group = c.benchmark_group("assess");
    group.bench_function("negative_income", |b| {
        b.iter(|| assess(black_box(year), black_box(&negative)).is_err())
    });
    group.finish();
}

fn surcharge(c: &mut Criterion) {
    let inputs: Vec<(TaxYear, FilingStatus, Cents)> = TaxYear::ALL
        .into_iter()
        .flat_map(|year| {
            tariff_zones(year).into_iter().flat_map(move |zone| {
                [FilingStatus::Single, FilingStatus::Joint]
                    .map(move |filing| (year, filing, Cents::from_euros(zone.from_eur)))
            })
        })
        .collect();
    let mut group = c.benchmark_group("solidarity_surcharge");
    group.throughput(Throughput::Elements(inputs.len() as u64));
    group.bench_function("tax_at_zone_starts", |b| {
        b.iter(|| {
            inputs.iter().fold(0i128, |sink, (year, filing, tax)| {
                sink ^ solidarity_surcharge(black_box(*year), black_box(*filing), black_box(*tax))
                    .map_or(0, |surcharge| surcharge.get())
            })
        })
    });
    group.finish();
}

criterion_group!(benches, assess_calls, explain_calls, error_path, surcharge);
criterion_main!(benches);
