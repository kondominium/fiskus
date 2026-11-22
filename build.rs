// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

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
    let rules = manifest_dir.join("rules");
    println!("cargo:rerun-if-changed={}", rules.display());

    let years = year_files(&rules)?;
    let mut generated = String::new();
    let mut tariffs = String::new();
    for year in &years {
        let file_path = rules.join(format!("{year}.toml"));
        println!("cargo:rerun-if-changed={}", file_path.display());
        let (parameters, zones, tariff_source) = load_year(&manifest_dir, &file_path)?;

        generated.push_str(&format!(
            "static PARAMETERS_{year}: Parameters = Parameters {{\n"
        ));
        for (key, parameter) in &parameters {
            let value_eur = parameter.value_eur;
            generated.push_str(&format!(
                "    {key}: Parameter {{ value_eur: {value_eur} }},\n"
            ));
        }
        generated.push_str("};\n");

        tariffs.push_str(&format!(
            "static TARIFF_SOURCE_{year}: Source = Source {{ citation: {:?}, source: {:?}, reference: {:?} }};\n",
            tariff_source.citation, tariff_source.source, tariff_source.reference
        ));
        tariffs.push_str(&format!(
            "static TARIFF_{year}: [Zone; {}] = [\n",
            zones.len()
        ));
        for zone in &zones {
            tariffs.push_str(&format!(
                "    Zone {{ from_eur: {}, base_eur: {}, step_eur: {}, quadratic_h: {}, linear_h: {}, constant_h: {} }},\n",
                zone.from_eur, zone.base_eur, zone.step_eur, zone.quadratic_h, zone.linear_h, zone.constant_h
            ));
        }
        tariffs.push_str("];\n");
    }

    generated.push_str("\npub(crate) fn for_year(year: TaxYear) -> &'static Parameters {\n");
    generated.push_str("    match year {\n");
    for year in &years {
        generated.push_str(&format!(
            "        TaxYear::Y{year} => &PARAMETERS_{year},\n"
        ));
    }
    generated.push_str("    }\n}\n\n");

    generated.push_str(&tariffs);
    generated.push_str("\npub(crate) fn tariff_source_for(year: TaxYear) -> &'static Source {\n");
    generated.push_str("    match year {\n");
    for year in &years {
        generated.push_str(&format!(
            "        TaxYear::Y{year} => &TARIFF_SOURCE_{year},\n"
        ));
    }
    generated.push_str("    }\n}\n");
    generated.push_str("\npub(crate) fn tariff_for(year: TaxYear) -> &'static [Zone] {\n");
    generated.push_str("    match year {\n");
    for year in &years {
        generated.push_str(&format!("        TaxYear::Y{year} => &TARIFF_{year},\n"));
    }
    generated.push_str("    }\n}\n");

    let target = out_dir.join("parameters.rs");
    fs::write(&target, generated).map_err(|e| format!("{}: {e}", target.display()))
}

fn year_files(rules: &Path) -> Result<Vec<u16>, String> {
    let entries = fs::read_dir(rules).map_err(|e| format!("{}: {e}", rules.display()))?;
    let mut years = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", rules.display()))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let stem = name.strip_suffix(".toml").unwrap_or("");
        if let Ok(year) = stem.parse::<u16>() {
            years.push(year);
        }
    }
    years.sort_unstable();
    Ok(years)
}

#[allow(clippy::type_complexity)]
fn load_year(
    manifest_dir: &Path,
    path: &Path,
) -> Result<(BTreeMap<String, Parameter>, Vec<ZoneDef>, Source), String> {
    #[derive(Deserialize)]
    struct YearFile {
        basic_allowance: Parameter,
        source: Source,
        zone: Vec<ZoneDef>,
    }

    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let year_file: YearFile =
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let fail = |reason: String| format!("{}: {reason}", path.display());

    let mut parameters = BTreeMap::new();
    parameters.insert("basic_allowance".to_string(), year_file.basic_allowance);

    for (key, parameter) in &parameters {
        check(manifest_dir, key, parameter)
            .map_err(|reason| format!("{}: [{key}] {reason}", path.display()))?;
    }

    let source = year_file.source;
    for (field, value) in [
        ("citation", &source.citation),
        ("source", &source.source),
        ("retrieved", &source.retrieved),
        ("reference", &source.reference),
    ] {
        if value.trim().is_empty() {
            return Err(fail(format!("[source] {field} is empty")));
        }
    }
    if !manifest_dir.join(&source.reference).is_file() {
        return Err(fail(format!(
            "reference {} does not exist",
            source.reference
        )));
    }

    let zones = year_file.zone;
    if zones.is_empty() {
        return Err(fail("no [[zone]] entries".into()));
    }

    let basic_allowance_eur = parameters
        .get("basic_allowance")
        .ok_or_else(|| fail("missing [basic_allowance]".into()))?
        .value_eur;

    if zones[0].from_eur != basic_allowance_eur + 1 {
        return Err(fail(format!(
            "first zone starts at {} but the basic allowance is {basic_allowance_eur}",
            zones[0].from_eur
        )));
    }
    for zone in &zones {
        if zone.step_eur <= 0 {
            return Err(fail(format!(
                "zone from {} has step_eur {}",
                zone.from_eur, zone.step_eur
            )));
        }
    }
    for pair in zones.windows(2) {
        if pair[1].from_eur <= pair[0].from_eur {
            return Err(fail(format!(
                "zone from {} is not after {}",
                pair[1].from_eur, pair[0].from_eur
            )));
        }
    }
    let mut previous_value = 0;
    let mut previous_zone: Option<&ZoneDef> = None;
    for zone in &zones {
        if let Some(prev) = previous_zone {
            let before = zone_value(prev, zone.from_eur - 1);
            let at = zone_value(zone, zone.from_eur);
            if (at - before).abs() > 1 {
                return Err(fail(format!(
                    "tax jumps at {}: {before} € before, {at} € at the start of the zone",
                    zone.from_eur
                )));
            }
        } else {
            let at = zone_value(zone, zone.from_eur);
            if (at - previous_value).abs() > 1 {
                return Err(fail("tax jumps out of the zero zone".into()));
            }
        }
        previous_value = zone_value(zone, zone.from_eur);
        previous_zone = Some(zone);
    }

    Ok((parameters, zones, source))
}

fn zone_value(zone: &ZoneDef, euros: i128) -> i128 {
    let d = euros - zone.base_eur;
    let s = zone.step_eur;
    let quadratic = if zone.quadratic_h == 0 {
        0
    } else {
        zone.quadratic_h * d * d
    };
    let numerator = quadratic + zone.linear_h * d * s + zone.constant_h * s * s;
    numerator.div_euclid(100 * s * s)
}

fn check(manifest_dir: &Path, key: &str, parameter: &Parameter) -> Result<(), String> {
    if !is_snake_case(key) {
        return Err("key must be lowercase snake_case".into());
    }
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
    let reference = manifest_dir.join(&parameter.reference);
    if !reference.is_file() {
        return Err(format!("reference {} does not exist", reference.display()));
    }
    Ok(())
}

fn is_snake_case(key: &str) -> bool {
    key.starts_with(|c: char| c.is_ascii_lowercase())
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|e| format!("{name}: {e}"))
}
