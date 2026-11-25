// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Draws the charts of the guide (`docs/guide/assets/charts`) from the public API.
//!
//! The charts use the latest listed year and inputs scaled to its basic allowance, so they follow
//! the rule files. `cargo run --example guide_charts` writes the SVG files. With `-- --check` it
//! writes nothing and fails if a committed file differs from what the code produces.
//!
//! Colors are CSS variables with light fallbacks, so the book's themes restyle each chart
//! (see `docs/theme/fiskus.css`). Each chart uses the signal color for one highlighted point only.

use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use fiskus::{
    Cents, Error, FilingStatus, FilingUnit, TaxYear, assess, basic_allowance_eur, explain,
    solidarity_free_threshold_eur, solidarity_surcharge, tariff_zones,
};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 360.0;
const LEFT: f64 = 104.0;
const RIGHT: f64 = 40.0;
const TOP: f64 = 48.0;
const BOTTOM: f64 = 80.0;
const FONT: &str = "Fira Sans, system-ui, sans-serif";
const INK: &str = "var(--chart-ink, #0f3d2e)";
const ZONE: &str = "var(--chart-zone, #f3f0e8)";
const SIGNAL: &str = "#e0391f";
const TICK: u32 = 14;
const LABEL: u32 = 15;

type Result<T> = std::result::Result<T, Box<dyn StdError>>;
type Render = fn() -> Result<String>;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("guide_charts: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let check = std::env::args().skip(1).any(|arg| arg == "--check");
    let docs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs");
    let charts: [(&str, Render); 8] = [
        ("rounding", rounding),
        ("zero-zone", zero_zone),
        ("tariff-rates", tariff_rates),
        ("solidarity-surcharge", solidarity_surcharge_chart),
        ("progressive-rates", progressive_rates),
        ("tax-by-income", tax_by_income),
        ("allowance-history", allowance_history),
        ("marginal-rate-solidarity", marginal_rate_with_solidarity),
    ];

    let mut rendered = charts
        .into_iter()
        .map(|(name, render)| {
            render().map(|svg| {
                (
                    docs.join("guide/assets/charts").join(format!("{name}.svg")),
                    svg,
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;
    rendered.push((
        docs.join("formulas/tariff-coefficients.md"),
        tariff_coefficients()?,
    ));
    rendered.push((
        docs.join("guide/assets/tables/tax-by-income.md"),
        tax_by_income_table()?,
    ));
    rendered.push((
        docs.join("guide/assets/tables/solidarity-breakdown.md"),
        solidarity_breakdown_table()?,
    ));

    if !check {
        for (path, content) in &rendered {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, content)?;
            println!("wrote {}", path.display());
        }
        return Ok(ExitCode::SUCCESS);
    }

    let stale: Vec<&PathBuf> = rendered
        .iter()
        .filter(|(path, content)| {
            fs::read_to_string(path).ok().as_deref() != Some(content.as_str())
        })
        .map(|(path, _)| path)
        .collect();
    for path in &stale {
        eprintln!("out of date: {}", path.display());
    }
    if stale.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    eprintln!("run `make charts` and commit the result");
    Ok(ExitCode::FAILURE)
}

/// The coefficient tables of `docs/formulas/tariff.md`, read from the same data as the code.
fn tariff_coefficients() -> Result<String> {
    let mut out = String::from(
        "<!-- Generated from rules/ by scripts/guide/charts.rs (make charts). Do not edit by hand. -->\n\n",
    );
    out += "| VZ | Basic allowance (€) | Tariff citation | Archived copy |\n|---|---|---|---|\n";
    for year in TaxYear::ALL {
        let explanation = explain(year, &unit(FilingStatus::Single, Cents::new(0)))?;
        let (citation, archived) = match explanation.source {
            Some(source) => (
                source.citation.to_string(),
                format!("[PDF]({})", source.reference),
            ),
            None => ("no tariff zones".to_string(), "—".to_string()),
        };
        out += &format!(
            "| {} | {} | {} | {} |\n",
            year.year(),
            group(basic_allowance_eur(year)),
            citation,
            archived
        );
    }

    out += "\nEach zone is `a·t² + b·t + c` with `t = (x − base) / step`. `x` is the whole-euro basis. The coefficients `a`, `b` and `c` are in euros.\n\n";
    out += "| VZ | Zone | From (€) | Base (€) | Step (€) | a | b | c |\n|---|---|---|---|---|---|---|---|\n";
    for year in TaxYear::ALL {
        let zones = tariff_zones(year);
        if zones.is_empty() {
            out += &format!("| {} | — | no zones | | | | | |\n", year.year());
        }
        for zone in zones {
            out += &format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                year.year(),
                zone.number,
                group(zone.from_eur),
                group(zone.base_eur),
                group(zone.step_eur),
                hundredths(zone.quadratic_h),
                hundredths(zone.linear_h),
                hundredths(zone.constant_h),
            );
        }
    }
    Ok(out)
}

/// A value in hundredths of a euro, written in euros with a point and no trailing zeros.
fn hundredths(value: i128) -> String {
    let sign = if value < 0 { "−" } else { "" };
    let magnitude = value.abs();
    match magnitude % 100 {
        0 => format!("{sign}{}", group(magnitude / 100)),
        cents => format!("{sign}{}.{cents:02}", group(magnitude / 100)),
    }
}

/// The year the charts show: the latest listed year.
fn sample_year() -> TaxYear {
    *TaxYear::ALL.last().expect("at least one year is listed")
}

fn unit(filing: FilingStatus, taxable_income: Cents) -> FilingUnit {
    FilingUnit {
        filing,
        taxable_income,
    }
}

fn rounding() -> Result<String> {
    let year = sample_year();
    let allowance = basic_allowance_eur(year);
    let start = (allowance - 1) * 100;
    let end = (allowance + 1) * 100 + 99;
    let points = (start..=end)
        .map(|cents| {
            assess(year, &unit(FilingStatus::Single, Cents::new(cents)))
                .map(|assessment| (cents, assessment.taxable_income.whole_euros()))
        })
        .collect::<std::result::Result<Vec<_>, Error>>()?;
    let (low, high) = bounds(points.iter().map(|&(_, euros)| euros));
    let plot = Plot {
        x: (start as f64, end as f64),
        y: (low as f64 - 0.5, high as f64 + 0.5),
    };

    let curve: Vec<(f64, f64)> = points
        .iter()
        .map(|&(cents, euros)| (plot.px(cents as f64), plot.py(euros as f64)))
        .collect();
    let mut body = axes() + &polyline(&curve, INK, 2.5);
    body += &(start..=end)
        .step_by(100)
        .map(|cents| {
            text(
                plot.px(cents as f64),
                HEIGHT - BOTTOM + 22.0,
                "middle",
                TICK,
                &euros_cents(cents),
            )
        })
        .collect::<String>();
    body += &[low, high]
        .into_iter()
        .map(|amount| {
            text(
                LEFT - 10.0,
                plot.py(amount as f64) + 5.0,
                "end",
                TICK,
                &euros(amount),
            )
        })
        .collect::<String>();

    let (dot_x, dot_y) = (plot.px(end as f64), plot.py(high as f64));
    body += &circle(dot_x, dot_y, 5.0, SIGNAL);
    body += &text(
        LEFT + 8.0,
        TOP + 16.0,
        "start",
        LABEL,
        &format!("{} is taken as {}", euros_cents(end), euros(high)),
    );
    body += &axis_titles(
        "Taxable income (zvE), to the cent",
        "Amount used for the tariff",
    );
    Ok(document(
        "Taxable income is rounded down to whole euros",
        &body,
    ))
}

fn zero_zone() -> Result<String> {
    let year = sample_year();
    let single_end = zero_tax_end(year, FilingStatus::Single)?;
    let joint_end = zero_tax_end(year, FilingStatus::Joint)?;
    let plot = Plot {
        x: (0.0, joint_end as f64),
        y: (0.0, 2.0),
    };

    let mut body = line(LEFT, HEIGHT - BOTTOM, WIDTH - RIGHT, HEIGHT - BOTTOM, INK);
    body += &bar(&plot, single_end, 1.2, 1.8, true);
    body += &bar(&plot, joint_end, 0.2, 0.8, false);
    body += &[("Single", single_end, 1.8), ("Joint", joint_end, 0.8)]
        .into_iter()
        .map(|(label, end, top)| {
            text(LEFT - 10.0, plot.py(top - 0.3) + 5.0, "end", LABEL, label)
                + &text(
                    plot.px(end as f64),
                    plot.py(top) - 10.0,
                    "end",
                    LABEL,
                    &euros(end),
                )
        })
        .collect::<String>();
    body += &circle(plot.px(joint_end as f64), plot.py(0.5), 6.0, SIGNAL);
    body += &(0..=joint_end)
        .step_by(4_000)
        .map(|amount| {
            text(
                plot.px(amount as f64),
                HEIGHT - BOTTOM + 22.0,
                "middle",
                TICK,
                &euros(amount),
            )
        })
        .collect::<String>();
    body += &axis_titles(
        "Taxable income (zvE), whole euros",
        "Zero tax up to this taxable income",
    );
    Ok(document(
        "The basic allowance is the zero-tax zone of the tariff",
        &body,
    ))
}

/// Largest whole-euro taxable income with zero tax, counted up from zero.
fn zero_tax_end(year: TaxYear, filing: FilingStatus) -> Result<i128> {
    let limit = basic_allowance_eur(year) * 3;
    let zero_tax = |euros: &i128| {
        assess(year, &unit(filing, Cents::from_euros(*euros)))
            .is_ok_and(|assessment| assessment.income_tax == Cents::new(0))
    };
    (0..=limit)
        .take_while(zero_tax)
        .last()
        .ok_or_else(|| "no zero-tax zone found".into())
}

fn tariff_rates() -> Result<String> {
    let year = sample_year();
    let max_income = basic_allowance_eur(year) * 10;
    let points = income_range(max_income)
        .map(|euros| {
            explain(year, &unit(FilingStatus::Single, Cents::from_euros(euros)))
                .map(|explanation| (euros, explanation.average_rate, explanation.marginal_rate))
        })
        .collect::<std::result::Result<Vec<_>, Error>>()?;
    let max_rate = points
        .iter()
        .map(|&(_, _, marginal)| marginal)
        .fold(0.0, f64::max);
    let top = (max_rate * 10.0).ceil() / 10.0;
    let plot = Plot {
        x: (0.0, max_income as f64),
        y: (0.0, top),
    };

    let average: Vec<(f64, f64)> = points
        .iter()
        .map(|&(euros, average, _)| (plot.px(euros as f64), plot.py(average)))
        .collect();
    let marginal: Vec<(f64, f64)> = points
        .iter()
        .map(|&(euros, _, marginal)| (plot.px(euros as f64), plot.py(marginal)))
        .collect();

    let mut body = axes() + &polyline(&average, INK, 2.5);
    body += &polyline_dashed(&marginal, INK, 2.5);
    body += &legend(
        LEFT + 12.0,
        TOP + 16.0,
        &[("Average rate", false), ("Marginal rate", true)],
    );
    body += &y_ticks(&plot, top, &percent);
    body += &x_ticks(&plot, max_income as f64, &|value| euros(value as i128));
    body += &axis_titles(
        "Taxable income (zvE), single filer, whole euros",
        &format!("Tax rate, VZ {}", year.year()),
    );
    Ok(document(
        "Average and marginal tax rate by taxable income",
        &body,
    ))
}

fn solidarity_surcharge_chart() -> Result<String> {
    let year = sample_year();
    let threshold_eur = solidarity_free_threshold_eur(year, FilingStatus::Single)
        .ok_or("the latest year has no solidarity threshold")?;
    let max_tax = threshold_eur * 2;
    let points = income_range(max_tax)
        .map(|euros| {
            solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(euros))
                .map(|surcharge| (euros, surcharge.whole_euros()))
                .ok_or("the latest year has no solidarity threshold")
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let max_surcharge = points.last().map_or(0, |&(_, surcharge)| surcharge);
    let plot = Plot {
        x: (0.0, max_tax as f64),
        y: (0.0, max_surcharge as f64),
    };
    let curve: Vec<(f64, f64)> = points
        .iter()
        .map(|&(tax, surcharge)| (plot.px(tax as f64), plot.py(surcharge as f64)))
        .collect();
    let (dot_x, dot_y) = (plot.px(max_tax as f64), plot.py(max_surcharge as f64));

    let mut body = axes() + &polyline(&curve, INK, 2.5);
    body += &y_ticks(&plot, max_surcharge as f64, &|value| euros(value as i128));
    body += &x_ticks(&plot, max_tax as f64, &|value| euros(value as i128));
    body += &circle(dot_x, dot_y, 5.0, SIGNAL);
    body += &text(
        dot_x - 12.0,
        dot_y - 14.0,
        "end",
        LABEL,
        &format!("{} tax, {} surcharge", euros(max_tax), euros(max_surcharge)),
    );
    body += &text(
        LEFT + 8.0,
        TOP + 16.0,
        "start",
        LABEL,
        &format!(
            "Free threshold, VZ {}: {}",
            year.year(),
            euros(threshold_eur)
        ),
    );
    body += &axis_titles("Income tax, whole euros", "Surcharge in euros");
    Ok(document(
        "The solidarity surcharge is zero up to the threshold",
        &body,
    ))
}

/// Marginal rate at the first euro of each progressive zone, for the latest year.
fn progressive_rates() -> Result<String> {
    let year = sample_year();
    let rates = tariff_zones(year)
        .into_iter()
        .map(|zone| {
            explain(
                year,
                &unit(FilingStatus::Single, Cents::from_euros(zone.from_eur)),
            )
            .map(|explanation| (zone.number, explanation.marginal_rate))
        })
        .collect::<std::result::Result<Vec<_>, Error>>()?;
    let top = rates.iter().map(|&(_, rate)| rate).fold(0.0, f64::max);
    let top = (top * 10.0).ceil() / 10.0;
    let plot = Plot {
        x: (0.0, rates.len() as f64),
        y: (0.0, top),
    };
    let slot = (WIDTH - LEFT - RIGHT) / rates.len() as f64;
    let mut body = axes();
    body += &y_ticks(&plot, top, &percent);
    for (index, (number, rate)) in rates.iter().enumerate() {
        let x = LEFT + slot * index as f64 + slot * 0.2;
        let top_y = plot.py(*rate);
        let height = plot.py(0.0) - top_y;
        body += &rect(x, top_y, slot * 0.6, height, INK);
        body += &text(
            x + slot * 0.3,
            top_y - 8.0,
            "middle",
            LABEL,
            &format!("{:.1} %", rate * 100.0),
        );
        body += &text(
            x + slot * 0.3,
            HEIGHT - BOTTOM + 22.0,
            "middle",
            TICK,
            &format!("Zone {number}"),
        );
    }
    body += &axis_titles(
        "Progressive zone of § 32a Abs. 1 EStG",
        &format!("Marginal rate at the zone's first euro, VZ {}", year.year()),
    );
    Ok(document(
        "Each progressive zone starts at a higher marginal rate",
        &body,
    ))
}

/// Income tax for a single filer from 1 € to 300 000 €, for the latest year.
fn tax_by_income() -> Result<String> {
    let year = sample_year();
    let max_income = 300_000;
    let points = income_range(max_income)
        .map(|euros| {
            assess(year, &unit(FilingStatus::Single, Cents::from_euros(euros)))
                .map(|assessment| (euros, assessment.income_tax.whole_euros()))
        })
        .collect::<std::result::Result<Vec<_>, Error>>()?;
    let max_tax = points.last().map_or(0, |&(_, tax)| tax);
    let plot = Plot {
        x: (0.0, max_income as f64),
        y: (0.0, max_tax as f64),
    };
    let curve: Vec<(f64, f64)> = points
        .iter()
        .map(|&(euros, tax)| (plot.px(euros as f64), plot.py(tax as f64)))
        .collect();
    let (dot_x, dot_y) = (plot.px(max_income as f64), plot.py(max_tax as f64));

    let mut body = axes() + &polyline(&curve, INK, 2.5);
    body += &y_ticks(&plot, max_tax as f64, &|value| euros(value as i128));
    body += &x_ticks(&plot, max_income as f64, &|value| euros(value as i128));
    body += &circle(dot_x, dot_y, 5.0, SIGNAL);
    body += &text(
        dot_x - 12.0,
        dot_y - 14.0,
        "end",
        LABEL,
        &format!("{} at {}", euros(max_tax), euros(max_income)),
    );
    body += &axis_titles(
        "Taxable income (zvE), single filer, whole euros",
        &format!("Income tax in euros, VZ {}", year.year()),
    );
    Ok(document(
        "Income tax from 1 € to 300 000 €, single filer",
        &body,
    ))
}

/// The basic allowance of every listed year, as steps.
fn allowance_history() -> Result<String> {
    let first = TaxYear::ALL[0].year();
    let last = TaxYear::ALL[TaxYear::ALL.len() - 1].year();
    let steps: Vec<(i128, i128)> = TaxYear::ALL
        .into_iter()
        .map(|year| (i128::from(year.year()), basic_allowance_eur(year)))
        .collect();
    let max = steps.iter().map(|&(_, eur)| eur).max().unwrap_or(0);
    let plot = Plot {
        x: (f64::from(first), f64::from(last) + 1.0),
        y: (0.0, max as f64),
    };
    let mut points = Vec::new();
    for (index, &(year, eur)) in steps.iter().enumerate() {
        let next = steps.get(index + 1).map_or(year + 1, |&(next, _)| next);
        points.push((plot.px(year as f64), plot.py(eur as f64)));
        points.push((plot.px(next as f64), plot.py(eur as f64)));
    }

    let mut body = axes() + &polyline(&points, INK, 2.5);
    body += &y_ticks(&plot, max as f64, &|value| euros(value as i128));
    body += &(i128::from(first)..=i128::from(last))
        .filter(|year| year % 5 == 0)
        .map(|year| {
            text(
                plot.px(year as f64),
                HEIGHT - BOTTOM + 22.0,
                "middle",
                TICK,
                &year.to_string(),
            )
        })
        .collect::<String>();
    body += &axis_titles("Assessment year (VZ)", "Basic allowance in euros");
    Ok(document("The basic allowance has risen in steps", &body))
}

/// Slope of the solidarity surcharge in tax terms at `tax_eur`: how much of each extra euro of
/// income tax is surcharged, measured over a window of 1 000 € so that the floors do not matter.
fn solidarity_slope(year: TaxYear, tax_eur: i128) -> Result<f64> {
    const WINDOW_EUR: i128 = 1_000;
    let surcharge_eur = |tax: i128| -> Result<f64> {
        let cents = solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(tax.max(0)))
            .ok_or("the latest year has no solidarity threshold")?;
        Ok(cents.get() as f64 / 100.0)
    };
    let lower = (tax_eur - WINDOW_EUR).max(0);
    let upper = tax_eur + WINDOW_EUR;
    Ok((surcharge_eur(upper)? - surcharge_eur(lower)?) / (upper - lower) as f64)
}

/// The smallest income for which `holds` is true, found by bisection over whole euros. `holds` must
/// be false below some income and true from there on.
fn first_income(max: i128, holds: impl Fn(i128) -> Result<bool>) -> Result<Option<i128>> {
    if !holds(max)? {
        return Ok(None);
    }
    let (mut low, mut high) = (0, max);
    while low < high {
        let middle = (low + high) / 2;
        if holds(middle)? {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    Ok(Some(low))
}

/// Income tax, solidarity surcharge and the points where the surcharge starts and reaches 5.5 %,
/// for a single filer and the latest year.
struct SolidarityBreakdown {
    threshold_tax_eur: i128,
    start_income_eur: Option<i128>,
    full_rate_income_eur: Option<i128>,
    full_rate_tax_eur: Option<i128>,
}

fn solidarity_breakdown(year: TaxYear, max_income: i128) -> Result<SolidarityBreakdown> {
    let threshold_tax_eur = solidarity_free_threshold_eur(year, FilingStatus::Single)
        .ok_or("the latest year has no solidarity threshold")?;
    let tax_at = |income: i128| -> Result<i128> {
        Ok(
            assess(year, &unit(FilingStatus::Single, Cents::from_euros(income)))?
                .income_tax
                .whole_euros(),
        )
    };
    let start_income_eur =
        first_income(max_income, |income| Ok(tax_at(income)? > threshold_tax_eur))?;
    let full_rate_income_eur = first_income(max_income, |income| {
        let tax = tax_at(income)?;
        let surcharge = solidarity_surcharge(year, FilingStatus::Single, Cents::from_euros(tax))
            .ok_or("the latest year has no solidarity threshold")?;
        Ok(surcharge.get() == Cents::from_euros(tax).get() * 11 / 200)
    })?;
    let full_rate_tax_eur = full_rate_income_eur.map(tax_at).transpose()?;
    Ok(SolidarityBreakdown {
        threshold_tax_eur,
        start_income_eur,
        full_rate_income_eur,
        full_rate_tax_eur,
    })
}

/// Marginal tax rate of a single filer with and without the solidarity surcharge, for the latest year.
fn marginal_rate_with_solidarity() -> Result<String> {
    let year = sample_year();
    let max_income = 300_000;
    let points = income_range(max_income)
        .map(|income| {
            let explanation =
                explain(year, &unit(FilingStatus::Single, Cents::from_euros(income)))?;
            let slope = solidarity_slope(year, explanation.income_tax_eur)?;
            Ok((
                income,
                explanation.marginal_rate,
                explanation.marginal_rate * (1.0 + slope),
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    let top = points.iter().map(|&(_, _, with)| with).fold(0.0, f64::max);
    let top = (top * 10.0).ceil() / 10.0;
    let plot = Plot {
        x: (0.0, max_income as f64),
        y: (0.0, top),
    };
    let excluding: Vec<(f64, f64)> = points
        .iter()
        .map(|&(income, without, _)| (plot.px(income as f64), plot.py(without)))
        .collect();
    let including: Vec<(f64, f64)> = points
        .iter()
        .map(|&(income, _, with)| (plot.px(income as f64), plot.py(with)))
        .collect();

    let breakdown = solidarity_breakdown(year, max_income)?;
    let mut body = axes();
    body += &polyline(&including, INK, 2.5);
    body += &polyline_dashed(&excluding, INK, 2.5);
    body += &legend(
        LEFT + 12.0,
        TOP + 16.0,
        &[
            ("Including solidarity", false),
            ("Excluding solidarity", true),
        ],
    );
    body += &y_ticks(&plot, top, &percent);
    body += &x_ticks(&plot, max_income as f64, &|value| euros(value as i128));
    if let Some(start) = breakdown.start_income_eur {
        let at_start = points
            .iter()
            .find(|&&(income, _, _)| income >= start)
            .map_or(0.0, |&(_, _, with)| with);
        body += &circle(plot.px(start as f64), plot.py(at_start), 5.0, SIGNAL);
        body += &text(
            plot.px(start as f64) + 10.0,
            plot.py(at_start) + 22.0,
            "start",
            LABEL,
            &format!("Surcharge starts at {}", euros(start)),
        );
    }
    if let Some(full) = breakdown.full_rate_income_eur {
        body += &text(
            plot.px(full as f64) + 4.0,
            plot.py(0.0) - 12.0,
            "start",
            LABEL,
            &format!("5.5 % from {}", euros(full)),
        );
    }
    body += &axis_titles(
        "Taxable income (zvE), single filer, whole euros",
        &format!("Marginal tax rate, VZ {}", year.year()),
    );
    Ok(document(
        "The solidarity surcharge adds to the marginal rate where it applies",
        &body,
    ))
}

/// Where the solidarity surcharge starts and reaches its full rate, for a single filer.
fn solidarity_breakdown_table() -> Result<String> {
    let year = sample_year();
    let breakdown = solidarity_breakdown(year, 300_000)?;
    let income = |value: Option<i128>| {
        value.map_or("not reached".to_string(), |euros| {
            format!("{} €", group(euros))
        })
    };
    let tax = |value: Option<i128>| {
        value.map_or("not reached".to_string(), |euros| {
            format!("{} €", group(euros))
        })
    };
    let mut out = String::from(
        "<!-- Generated from rules/ by scripts/guide/charts.rs (make charts). Do not edit by hand. -->\n\n",
    );
    out += "| Point | Income tax (€) | Taxable income, single filer (€) |\n|---|---|---|\n";
    out += &format!(
        "| Free threshold (Freigrenze) of § 3 Abs. 3 SolZG | {} | {} |\n",
        group(breakdown.threshold_tax_eur),
        income(breakdown.start_income_eur)
    );
    out += &format!(
        "| Full 5.5 % rate from (mitigation zone ends) | {} | {} |\n",
        tax(breakdown.full_rate_tax_eur),
        income(breakdown.full_rate_income_eur)
    );
    Ok(out)
}

/// A filled or outlined rectangle in pixels.
fn rect(x: f64, y: f64, width: f64, height: f64, ink: &str) -> String {
    format!(
        "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" style=\"fill: {ink}\"/>\n"
    )
}

/// Income tax and rates for a single filer at selected incomes, for the latest year.
fn tax_by_income_table() -> Result<String> {
    let year = sample_year();
    let mut out = String::from(
        "<!-- Generated from rules/ by scripts/guide/charts.rs (make charts). Do not edit by hand. -->\n\n",
    );
    out += "| Taxable income, single filer (€) | Income tax (€) | Average rate | Marginal rate |\n|---|---|---|---|\n";
    for euros in [
        1, 10_000, 20_000, 30_000, 50_000, 100_000, 150_000, 200_000, 250_000, 300_000,
    ] {
        let explanation = explain(year, &unit(FilingStatus::Single, Cents::from_euros(euros)))?;
        out += &format!(
            "| {} | {} | {:.1} % | {:.1} % |\n",
            group(euros),
            group(explanation.income_tax_eur),
            explanation.average_rate * 100.0,
            explanation.marginal_rate * 100.0,
        );
    }
    Ok(out)
}

/// Whole-euro points from zero to `max`, at most about 400 of them.
fn income_range(max: i128) -> impl Iterator<Item = i128> {
    let step = (max / 400).max(1);
    (0..=max)
        .step_by(step as usize)
        .chain([max])
        .filter(move |euros| *euros <= max)
}

fn bounds(values: impl Iterator<Item = i128>) -> (i128, i128) {
    values.fold((i128::MAX, i128::MIN), |(low, high), value| {
        (low.min(value), high.max(value))
    })
}

/// A tick step of 1, 2 or 5 times a power of ten, so that `max` takes about `ticks` steps.
fn nice_step(max: f64, ticks: f64) -> f64 {
    let raw = (max / ticks).max(f64::MIN_POSITIVE);
    let magnitude = 10f64.powf(raw.log10().floor());
    let scaled = raw / magnitude;
    let factor = [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .find(|factor| scaled <= *factor)
        .unwrap_or(10.0);
    factor * magnitude
}

fn y_ticks(plot: &Plot, max: f64, label: &dyn Fn(f64) -> String) -> String {
    let step = nice_step(max, 4.0);
    (0..)
        .map(|index| step * index as f64)
        .take_while(|value| *value <= max + f64::EPSILON)
        .map(|value| {
            text(
                LEFT - 10.0,
                plot.py(value) + 5.0,
                "end",
                TICK,
                &label(value),
            )
        })
        .collect()
}

fn x_ticks(plot: &Plot, max: f64, label: &dyn Fn(f64) -> String) -> String {
    let step = nice_step(max, 4.0);
    (0..)
        .map(|index| step * index as f64)
        .take_while(|value| *value <= max + f64::EPSILON)
        .map(|value| {
            text(
                plot.px(value),
                HEIGHT - BOTTOM + 22.0,
                "middle",
                TICK,
                &label(value),
            )
        })
        .collect()
}

fn legend(x: f64, y: f64, entries: &[(&str, bool)]) -> String {
    entries
        .iter()
        .enumerate()
        .map(|(index, (label, dashed))| {
            let row = y + index as f64 * 24.0;
            let stroke = if *dashed {
                polyline_dashed(&[(x, row), (x + 28.0, row)], INK, 2.5)
            } else {
                polyline(&[(x, row), (x + 28.0, row)], INK, 2.5)
            };
            stroke + &text(x + 36.0, row + 5.0, "start", TICK, label)
        })
        .collect()
}

fn euros(value: i128) -> String {
    format!("{} €", group(value))
}

fn euros_cents(cents: i128) -> String {
    format!(
        "{}.{:02} €",
        group(cents.div_euclid(100)),
        cents.rem_euclid(100)
    )
}

fn percent(value: f64) -> String {
    format!("{:.0} %", value * 100.0)
}

fn group(value: i128) -> String {
    let digits = value.abs().to_string();
    let grouped = digits
        .chars()
        .enumerate()
        .flat_map(|(index, digit)| {
            let space = (index > 0 && (digits.len() - index).is_multiple_of(3)).then_some(' ');
            space.into_iter().chain(std::iter::once(digit))
        })
        .collect::<String>();
    if value < 0 {
        format!("−{grouped}")
    } else {
        grouped
    }
}

struct Plot {
    x: (f64, f64),
    y: (f64, f64),
}

impl Plot {
    fn px(&self, value: f64) -> f64 {
        LEFT + (value - self.x.0) / (self.x.1 - self.x.0) * (WIDTH - LEFT - RIGHT)
    }

    fn py(&self, value: f64) -> f64 {
        HEIGHT - BOTTOM - (value - self.y.0) / (self.y.1 - self.y.0) * (HEIGHT - TOP - BOTTOM)
    }
}

fn document(title: &str, body: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {WIDTH} {HEIGHT}\" width=\"{WIDTH}\" height=\"{HEIGHT}\" font-family=\"{FONT}\" role=\"img\">\n<title>{title}</title>\n{body}</svg>\n"
    )
}

fn axes() -> String {
    line(LEFT, HEIGHT - BOTTOM, WIDTH - RIGHT, HEIGHT - BOTTOM, INK)
        + &line(LEFT, TOP, LEFT, HEIGHT - BOTTOM, INK)
}

fn axis_titles(x_title: &str, y_title: &str) -> String {
    text(
        (LEFT + WIDTH - RIGHT) / 2.0,
        HEIGHT - 18.0,
        "middle",
        LABEL,
        x_title,
    ) + &text(LEFT, TOP - 20.0, "start", LABEL, y_title)
}

fn line(x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str) -> String {
    format!(
        "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" style=\"stroke: {stroke}\" stroke-width=\"1\"/>\n"
    )
}

fn polyline(points: &[(f64, f64)], stroke: &str, width: f64) -> String {
    polyline_with(points, stroke, width, "")
}

fn polyline_dashed(points: &[(f64, f64)], stroke: &str, width: f64) -> String {
    polyline_with(points, stroke, width, " stroke-dasharray=\"7 5\"")
}

fn polyline_with(points: &[(f64, f64)], stroke: &str, width: f64, extra: &str) -> String {
    let coords: Vec<String> = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect();
    format!(
        "<polyline points=\"{}\" fill=\"none\" style=\"stroke: {stroke}\" stroke-width=\"{width}\" stroke-linejoin=\"round\"{extra}/>\n",
        coords.join(" ")
    )
}

fn circle(x: f64, y: f64, radius: f64, fill: &str) -> String {
    format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{radius}\" style=\"fill: {fill}\"/>\n")
}

fn text(x: f64, y: f64, anchor: &str, size: u32, content: &str) -> String {
    format!(
        "<text x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\" font-size=\"{size}\" style=\"fill: {INK}\">{content}</text>\n"
    )
}

fn bar(plot: &Plot, end: i128, bottom: f64, top: f64, filled: bool) -> String {
    let (x, y) = (plot.px(0.0), plot.py(top));
    let width = plot.px(end as f64) - x;
    let height = plot.py(bottom) - y;
    let style = if filled {
        format!("fill: {INK}")
    } else {
        format!("fill: {ZONE}; stroke: {INK}")
    };
    let stroke_width = if filled { "" } else { " stroke-width=\"1.5\"" };
    format!(
        "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" style=\"{style}\"{stroke_width}/>\n"
    )
}
