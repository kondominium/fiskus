// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! Build script: reads `rules/<year>.toml`, checks every value, and writes the tables that
//! `src/params.rs` includes. A bad file fails the build, and the message names the file and the reason.
#![expect(
    clippy::panic,
    reason = "a build script fails the build by panicking, and Cargo prints the message"
)]

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

const YEARS: [u16; 24] = [
    2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018,
    2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026,
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct YearFile {
    basic_allowance: Parameter,
    source: Option<Source>,
    #[serde(default)]
    zone: Vec<ZoneDef>,
    solidarity: Option<SolidarityDef>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SolidarityDef {
    free_threshold_single_eur: i128,
    free_threshold_joint_eur: i128,
    citation: String,
    source: String,
    retrieved: String,
    reference: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Parameter {
    value_eur: i128,
    citation: String,
    source: String,
    retrieved: String,
    reference: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[expect(
    clippy::struct_field_names,
    reason = "the fields are the keys of the [source] table in the rules files"
)]
struct Source {
    citation: String,
    source: String,
    retrieved: String,
    reference: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ZoneDef {
    from_eur: i128,
    base_eur: i128,
    step_eur: i128,
    quadratic_h: i128,
    linear_h: i128,
    constant_h: i128,
}

fn main() {
    if let Err(reason) = generate() {
        panic!("{reason}");
    }
}

fn generate() -> Result<(), String> {
    let manifest_dir = PathBuf::from(required_env("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(required_env("OUT_DIR")?);

    let mut generated = String::new();
    let mut tariffs = String::new();
    let mut sources = String::new();
    let mut has_source = Vec::new();
    let mut solidarity_tables = String::new();
    let mut has_solidarity = Vec::new();
    for year in YEARS {
        let path = manifest_dir.join(format!("rules/{year}.toml"));
        println!("cargo:rerun-if-changed={}", path.display());
        let file = load(&path)?;

        let _ = writeln!(
            generated,
            "static PARAMETERS_{year}: Parameters = Parameters {{\n    basic_allowance: Parameter {{ value_eur: {} }},\n}};",
            lit(file.basic_allowance.value_eur)
        );

        has_source.push((year, file.source.is_some()));
        if let Some(source) = &file.source {
            let _ = writeln!(
                sources,
                "static SOURCE_{year}: Source = Source {{ citation: {:?}, source: {:?}, reference: {:?} }};",
                source.citation, source.source, source.reference
            );
        }

        if let Some(solidarity) = &file.solidarity {
            check_solidarity(solidarity)
                .map_err(|reason| format!("rules/{year}.toml: [solidarity] {reason}"))?;
            let _ = writeln!(
                solidarity_tables,
                "static SOLIDARITY_{year}: Solidarity = Solidarity {{ free_threshold_single_eur: {}, free_threshold_joint_eur: {}, source: Source {{ citation: {:?}, source: {:?}, reference: {:?} }} }};",
                lit(solidarity.free_threshold_single_eur),
                lit(solidarity.free_threshold_joint_eur),
                solidarity.citation,
                solidarity.source,
                solidarity.reference
            );
        }
        has_solidarity.push((year, file.solidarity.is_some()));

        let _ = writeln!(
            tariffs,
            "static TARIFF_{year}: [Zone; {}] = [",
            file.zone.len()
        );
        for zone in &file.zone {
            let _ = writeln!(
                tariffs,
                "    Zone {{ from_eur: {}, base_eur: {}, step_eur: {}, quadratic_h: {}, linear_h: {}, constant_h: {} }},",
                lit(zone.from_eur),
                lit(zone.base_eur),
                lit(zone.step_eur),
                lit(zone.quadratic_h),
                lit(zone.linear_h),
                lit(zone.constant_h)
            );
        }
        tariffs.push_str("];\n");
    }

    lookup_fn(
        &mut generated,
        "",
        "for_year",
        "&'static Parameters",
        YEARS.map(|year| (year, format!("&PARAMETERS_{year}"))),
    );
    generated.push_str(&tariffs);
    lookup_fn(
        &mut generated,
        "",
        "tariff_for",
        "&'static [Zone]",
        YEARS.map(|year| (year, format!("&TARIFF_{year}"))),
    );
    generated.push_str(&sources);
    generated.push_str(&solidarity_tables);
    lookup_fn(
        &mut generated,
        "#[allow(clippy::match_same_arms, reason = \"the years without a threshold share the None arm\")]\n",
        "solidarity_for",
        "Option<&'static Solidarity>",
        optional_arms(has_solidarity, "SOLIDARITY"),
    );
    lookup_fn(
        &mut generated,
        "",
        "tariff_source_for",
        "Option<&'static Source>",
        optional_arms(has_source, "SOURCE"),
    );

    let target = out_dir.join("parameters.rs");
    fs::write(&target, generated).map_err(|e| format!("{}: {e}", target.display()))
}

/// Writes `pub(crate) fn name(year) -> return_type`, a `match` with one arm per year. `attribute`
/// goes above the function, and is empty when the function needs none.
fn lookup_fn(
    out: &mut String,
    attribute: &str,
    name: &str,
    return_type: &str,
    arms: impl IntoIterator<Item = (u16, String)>,
) {
    let _ = writeln!(
        out,
        "\n{attribute}pub(crate) fn {name}(year: TaxYear) -> {return_type} {{\n    match year {{"
    );
    for (year, value) in arms {
        let _ = writeln!(out, "        TaxYear::Y{year} => {value},");
    }
    out.push_str("    }\n}\n");
}

/// One arm per year: `Some(&{table}_{year})` when the year has that table, `None` otherwise.
fn optional_arms(
    years: Vec<(u16, bool)>,
    table: &'static str,
) -> impl Iterator<Item = (u16, String)> {
    years.into_iter().map(move |(year, present)| {
        let value = if present {
            format!("Some(&{table}_{year})")
        } else {
            "None".to_string()
        };
        (year, value)
    })
}

/// Writes an integer as a Rust literal with digit groups, so `12348` becomes `12_348`.
fn lit(value: i128) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push('_');
        }
        grouped.push(digit);
    }
    if value < 0 {
        format!("-{grouped}")
    } else {
        grouped
    }
}

fn check_solidarity(solidarity: &SolidarityDef) -> Result<(), String> {
    if solidarity.free_threshold_single_eur < 0 || solidarity.free_threshold_joint_eur < 0 {
        return Err("a free threshold is negative".into());
    }
    if solidarity.free_threshold_joint_eur < solidarity.free_threshold_single_eur {
        return Err("the joint threshold is below the single one".into());
    }
    for (field, text) in [
        ("citation", &solidarity.citation),
        ("source", &solidarity.source),
        ("retrieved", &solidarity.retrieved),
        ("reference", &solidarity.reference),
    ] {
        if text.trim().is_empty() {
            return Err(format!("{field} is empty"));
        }
    }
    Ok(())
}

fn load(path: &Path) -> Result<YearFile, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let file: YearFile = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let fail = |reason: String| format!("{}: {reason}", path.display());

    check_parameter(&file.basic_allowance)
        .map_err(|reason| fail(format!("[basic_allowance] {reason}")))?;
    let allowance_eur = file.basic_allowance.value_eur;

    match (&file.source, file.zone.is_empty()) {
        (None, true) => {}
        (None, false) => return Err(fail("zones given without a [source] table".into())),
        (Some(_), true) => return Err(fail("[source] given without any [[zone]] entries".into())),
        (Some(source), false) => {
            check_source(source).map_err(|reason| fail(format!("[source] {reason}")))?;
        }
    }
    check_zones(&file.zone, allowance_eur).map_err(fail)?;
    Ok(file)
}

fn check_parameter(parameter: &Parameter) -> Result<(), String> {
    if parameter.value_eur < 0 {
        return Err(format!("value_eur {} is negative", parameter.value_eur));
    }
    for (field, text) in [
        ("citation", &parameter.citation),
        ("source", &parameter.source),
        ("retrieved", &parameter.retrieved),
        ("reference", &parameter.reference),
    ] {
        if text.trim().is_empty() {
            return Err(format!("{field} is empty"));
        }
    }
    Ok(())
}

fn check_source(source: &Source) -> Result<(), String> {
    for (field, text) in [
        ("citation", &source.citation),
        ("source", &source.source),
        ("retrieved", &source.retrieved),
        ("reference", &source.reference),
    ] {
        if text.trim().is_empty() {
            return Err(format!("{field} is empty"));
        }
    }
    Ok(())
}

fn check_zones(zones: &[ZoneDef], allowance_eur: i128) -> Result<(), String> {
    let Some(first) = zones.first() else {
        return Ok(());
    };
    if first.from_eur != allowance_eur + 1 {
        return Err(format!(
            "first zone starts at {} but the basic allowance is {allowance_eur}",
            first.from_eur
        ));
    }
    for zone in zones {
        if zone.step_eur <= 0 {
            return Err(format!(
                "zone from {} has step_eur {}",
                zone.from_eur, zone.step_eur
            ));
        }
    }
    for (previous, zone) in zones.iter().zip(zones.iter().skip(1)) {
        if zone.from_eur <= previous.from_eur {
            return Err(format!(
                "zone from {} is not after {}",
                zone.from_eur, previous.from_eur
            ));
        }
    }
    let mut previous: Option<&ZoneDef> = None;
    for zone in zones {
        let at_start = zone_value(zone, zone.from_eur);
        let before = match previous {
            Some(prev) => zone_value(prev, zone.from_eur - 1),
            None => 0,
        };
        if (at_start - before).abs() > 1 {
            return Err(format!(
                "tax jumps at {}: {before} € before, {at_start} € at the start of the zone",
                zone.from_eur
            ));
        }
        previous = Some(zone);
    }
    Ok(())
}

fn zone_value(zone: &ZoneDef, euros: i128) -> i128 {
    let d = euros - zone.base_eur;
    let s = zone.step_eur;
    let numerator = zone.quadratic_h * d * d + zone.linear_h * d * s + zone.constant_h * s * s;
    numerator.div_euclid(100 * s * s)
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|e| format!("{name}: {e}"))
}
