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

    let years = year_directories(&rules)?;
    let mut generated = String::new();
    for year in &years {
        let path = rules.join(year.to_string()).join("parameters.toml");
        println!("cargo:rerun-if-changed={}", path.display());
        let parameters = load(&manifest_dir, &path)?;

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
    }

    generated.push_str("\npub(crate) fn for_year(year: TaxYear) -> &'static Parameters {\n");
    generated.push_str("    match year {\n");
    for year in &years {
        generated.push_str(&format!(
            "        TaxYear::Y{year} => &PARAMETERS_{year},\n"
        ));
    }
    generated.push_str("    }\n}\n");

    let target = out_dir.join("parameters.rs");
    fs::write(&target, generated).map_err(|e| format!("{}: {e}", target.display()))
}

fn year_directories(rules: &Path) -> Result<Vec<u16>, String> {
    let entries = fs::read_dir(rules).map_err(|e| format!("{}: {e}", rules.display()))?;
    let mut years = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", rules.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Ok(year) = name.parse::<u16>() {
            years.push(year);
        }
    }
    years.sort_unstable();
    Ok(years)
}

fn load(manifest_dir: &Path, path: &Path) -> Result<BTreeMap<String, Parameter>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let parameters: BTreeMap<String, Parameter> =
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    for (key, parameter) in &parameters {
        check(manifest_dir, key, parameter)
            .map_err(|reason| format!("{}: [{key}] {reason}", path.display()))?;
    }
    Ok(parameters)
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
