// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

const YEARS: [u16; 24] = [
    2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018,
    2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026,
];

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

    let mut generated = String::new();
    for year in YEARS {
        let path = manifest_dir.join(format!("rules/{year}.toml"));
        println!("cargo:rerun-if-changed={}", path.display());
        let parameters = load(&path)?;

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

    let target = out_dir.join("parameters.rs");
    fs::write(&target, generated).map_err(|e| format!("{}: {e}", target.display()))
}

fn load(path: &Path) -> Result<BTreeMap<String, Parameter>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let parameters: BTreeMap<String, Parameter> =
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    for (key, parameter) in &parameters {
        check(key, parameter).map_err(|reason| format!("{}: [{key}] {reason}", path.display()))?;
    }
    Ok(parameters)
}

fn check(key: &str, parameter: &Parameter) -> Result<(), String> {
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
