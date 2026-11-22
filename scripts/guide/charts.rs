// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Draws the charts of the guide (`docs/guide/assets/charts`) from the public `assess` API.
//!
//! `cargo run --example guide_charts` writes the SVG files. With `-- --check` it writes nothing
//! and fails if a committed file differs from what the code produces.
//!
//! Colors are CSS variables with light fallbacks, so the book's themes restyle each chart
//! (see `docs/theme/fiskus.css`). The guide pulls the files in with `{{#include}}`.

use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use fiskus::{Cents, Error, FilingStatus, FilingUnit, TaxYear, assess};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 360.0;
const LEFT: f64 = 96.0;
const RIGHT: f64 = 32.0;
const TOP: f64 = 40.0;
const BOTTOM: f64 = 72.0;
const FONT: &str = "Fira Sans, system-ui, sans-serif";
const INK: &str = "var(--chart-ink, #0f3d2e)";
const ZONE: &str = "var(--chart-zone, #f3f0e8)";
const SIGNAL: &str = "#e0391f";

type Render = fn() -> Result<String, Box<dyn StdError>>;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("guide_charts: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn StdError>> {
    let check = std::env::args().skip(1).any(|arg| arg == "--check");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/guide/assets/charts");
    let charts: [(&str, Render); 2] = [("rounding", rounding), ("zero-zone", zero_zone)];

    let mut stale = Vec::new();
    for (name, render) in charts {
        let path = dir.join(format!("{name}.svg"));
        let svg = render()?;
        if check {
            if fs::read_to_string(&path).ok().as_deref() != Some(svg.as_str()) {
                stale.push(path);
            }
        } else {
            fs::create_dir_all(&dir)?;
            fs::write(&path, svg)?;
            println!("wrote {}", path.display());
        }
    }

    if stale.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    for path in &stale {
        eprintln!("out of date: {}", path.display());
    }
    eprintln!("run `mise run charts` and commit the result");
    Ok(ExitCode::FAILURE)
}

fn rounding() -> Result<String, Box<dyn StdError>> {
    const START: i128 = 1_234_700;
    let mut points = Vec::new();
    for cents in START..START + 300 {
        match assess(
            TaxYear::Y2026,
            &unit(FilingStatus::Single, Cents::new(cents)),
        ) {
            Ok(assessment) => points.push((cents, assessment.taxable_income.whole_euros())),
            Err(Error::OutsideImplementedRange { .. }) => break,
            Err(error) => return Err(error.into()),
        }
    }
    let &(end, end_euros) = points.last().ok_or("no amounts in the implemented range")?;
    let (low, high) = points
        .iter()
        .fold((i128::MAX, i128::MIN), |(low, high), &(_, euros)| {
            (low.min(euros), high.max(euros))
        });

    let plot = Plot {
        x: (START as f64, end as f64),
        y: (low as f64 - 0.5, high as f64 + 0.5),
    };
    let curve: Vec<(f64, f64)> = points
        .iter()
        .map(|&(cents, euros)| (plot.px(cents as f64), plot.py(euros as f64)))
        .collect();

    let mut body = x_axis() + &y_axis() + &polyline(&curve, INK, 2.5);

    let mut ticks: Vec<i128> = (START..=end).step_by(50).collect();
    if ticks.last() != Some(&end) {
        ticks.push(end);
    }
    for cents in ticks {
        let anchor = if cents == end { "end" } else { "middle" };
        body += &text(
            plot.px(cents as f64),
            HEIGHT - BOTTOM + 20.0,
            anchor,
            12,
            &euros_cents(cents),
        );
    }
    for level in [low, high] {
        body += &text(
            LEFT - 10.0,
            plot.py(level as f64) + 4.0,
            "end",
            12,
            &euros(level),
        );
    }

    let (dot_x, dot_y) = (plot.px(end as f64), plot.py(end_euros as f64));
    body += &circle(dot_x, dot_y, 5.0, SIGNAL);
    body += &text(
        dot_x - 12.0,
        dot_y - 14.0,
        "end",
        13,
        &format!("{} is taken as {}", euros_cents(end), euros(end_euros)),
    );
    body += &text(
        WIDTH / 2.0,
        HEIGHT - 16.0,
        "middle",
        13,
        "Taxable income (zvE), to the cent",
    );
    body += &text(
        LEFT,
        TOP - 16.0,
        "start",
        13,
        "Amount used for the tariff, whole euros",
    );

    Ok(document(
        "Taxable income is rounded down to whole euros",
        &body,
    ))
}

fn zero_zone() -> Result<String, Box<dyn StdError>> {
    let single_end = zero_zone_end(FilingStatus::Single)?;
    let joint_end = zero_zone_end(FilingStatus::Joint)?;
    let plot = Plot {
        x: (0.0, joint_end as f64),
        y: (0.0, 2.0),
    };

    let mut body = x_axis();
    body += &bar(&plot, single_end, 1.2, 1.8, true);
    body += &bar(&plot, joint_end, 0.2, 0.8, false);

    for (label, end, top) in [("Single", single_end, 1.8), ("Joint", joint_end, 0.8)] {
        body += &text(LEFT - 10.0, plot.py(top - 0.3) + 4.0, "end", 13, label);
        body += &text(
            plot.px(end as f64),
            plot.py(top) - 8.0,
            "end",
            13,
            &euros(end),
        );
    }

    body += &circle(plot.px(joint_end as f64), plot.py(0.5), 6.0, SIGNAL);

    for amount in (0..=joint_end).step_by(4_000) {
        body += &text(
            plot.px(amount as f64),
            HEIGHT - BOTTOM + 20.0,
            "middle",
            12,
            &euros(amount),
        );
    }
    body += &text(
        WIDTH / 2.0,
        HEIGHT - 16.0,
        "middle",
        13,
        "Taxable income (zvE), whole euros",
    );
    body += &text(
        LEFT,
        TOP - 16.0,
        "start",
        13,
        "Zero tax up to this taxable income",
    );

    Ok(document(
        "The basic allowance is the zero-tax zone of the tariff",
        &body,
    ))
}

/// Largest whole-euro taxable income that `assess` accepts, found by stepping up from zero.
fn zero_zone_end(filing: FilingStatus) -> Result<i128, Box<dyn StdError>> {
    let mut last = None;
    for euros in 0..=100_000 {
        match assess(TaxYear::Y2026, &unit(filing, Cents::from_euros(euros))) {
            Ok(_) => last = Some(euros),
            Err(Error::OutsideImplementedRange { .. }) => break,
            Err(error) => return Err(error.into()),
        }
    }
    last.ok_or_else(|| "no zero-tax zone found".into())
}

fn unit(filing: FilingStatus, taxable_income: Cents) -> FilingUnit {
    FilingUnit {
        filing,
        taxable_income,
    }
}

fn group(value: i128) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(digit);
    }
    out
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

fn x_axis() -> String {
    line(LEFT, HEIGHT - BOTTOM, WIDTH - RIGHT, HEIGHT - BOTTOM, INK)
}

fn y_axis() -> String {
    line(LEFT, TOP, LEFT, HEIGHT - BOTTOM, INK)
}

fn line(x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str) -> String {
    format!(
        "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" style=\"stroke: {stroke}\" stroke-width=\"1\"/>\n"
    )
}

fn polyline(points: &[(f64, f64)], stroke: &str, width: f64) -> String {
    let coords: Vec<String> = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect();
    format!(
        "<polyline points=\"{}\" fill=\"none\" style=\"stroke: {stroke}\" stroke-width=\"{width}\" stroke-linejoin=\"round\"/>\n",
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
    if filled {
        format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" style=\"fill: {INK}\"/>\n"
        )
    } else {
        format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{width:.1}\" height=\"{height:.1}\" style=\"fill: {ZONE}; stroke: {INK}\" stroke-width=\"1.5\"/>\n"
        )
    }
}
