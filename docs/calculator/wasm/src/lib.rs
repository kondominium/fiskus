// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
//! JSON bridge for the calculator. Operations: `meta`, `explain`, `zones`, `curve` and `compare`.
//!
//! The host writes a JSON request into memory obtained from `fiskus_alloc`, calls `fiskus_call`,
//! reads the JSON answer from the returned pointer and length, and frees both with `fiskus_free`.

use std::slice;

use fiskus::{
    assess, basic_allowance_eur, explain, tariff_zones, Cents, Explanation, FilingStatus,
    FilingUnit, TaxYear, ZoneStep,
};
use serde_json::{json, Value};

fn tax_year(value: &Value) -> Result<TaxYear, String> {
    let year = value["year"].as_u64().ok_or("year must be a number")?;
    TaxYear::ALL
        .iter()
        .copied()
        .find(|candidate| u64::from(candidate.year()) == year)
        .ok_or_else(|| format!("no tax year {year}"))
}

fn filing(value: &Value) -> Result<FilingStatus, String> {
    match value["filing"].as_str() {
        Some("single") => Ok(FilingStatus::Single),
        Some("joint") => Ok(FilingStatus::Joint),
        _ => Err("filing must be \"single\" or \"joint\"".into()),
    }
}

fn filing_name(filing: FilingStatus) -> &'static str {
    match filing {
        FilingStatus::Single => "single",
        FilingStatus::Joint => "joint",
    }
}

fn unit(value: &Value, euros: i128) -> Result<FilingUnit, String> {
    Ok(FilingUnit {
        filing: filing(value)?,
        taxable_income: Cents::from_euros(euros),
    })
}

fn euros(value: &Value) -> Result<i128, String> {
    value["euros"]
        .as_i64()
        .map(i128::from)
        .ok_or_else(|| "euros must be a whole number".to_string())
}

fn zone_json(zone: &ZoneStep) -> Value {
    json!({
        "number": zone.number,
        "from_eur": zone.from_eur,
        "base_eur": zone.base_eur,
        "step_eur": zone.step_eur,
        "quadratic": zone.quadratic_h as f64 / 100.0,
        "linear": zone.linear_h as f64 / 100.0,
        "constant": zone.constant_h as f64 / 100.0,
    })
}

fn explanation_json(explanation: &Explanation) -> Value {
    json!({
        "year": explanation.year.year(),
        "filing": filing_name(explanation.filing),
        "taxable_income_eur": explanation.taxable_income_eur,
        "basis_eur": explanation.basis_eur,
        "basic_allowance_eur": explanation.basic_allowance_eur,
        "zone": explanation.zone.as_ref().map(zone_json),
        "unrounded_basis_tax": explanation.unrounded_basis_tax.numerator as f64
            / explanation.unrounded_basis_tax.denominator as f64,
        "basis_tax_eur": explanation.basis_tax_eur,
        "income_tax_eur": explanation.income_tax_eur,
        "average_rate": explanation.average_rate,
        "marginal_rate": explanation.marginal_rate,
        "citation": explanation.citation,
        "source": explanation.source,
        "reference": explanation.reference,
    })
}

fn respond(request: &Value) -> Result<Value, String> {
    match request["op"].as_str() {
        Some("meta") => {
            let years: Vec<Value> = TaxYear::ALL
                .iter()
                .map(|year| {
                    json!({
                        "year": year.year(),
                        "basic_allowance_eur": basic_allowance_eur(*year),
                    })
                })
                .collect();
            Ok(json!({ "years": years, "filings": ["single", "joint"] }))
        }
        Some("explain") => {
            let year = tax_year(request)?;
            let euros = euros(request)?;
            let explanation = explain(year, &unit(request, euros)?).map_err(|e| e.to_string())?;
            Ok(explanation_json(&explanation))
        }
        Some("zones") => {
            let year = tax_year(request)?;
            let zones: Vec<Value> = tariff_zones(year).iter().map(zone_json).collect();
            let source = explain(year, &unit(request, 0)?).map_err(|e| e.to_string())?;
            Ok(json!({
                "year": year.year(),
                "basic_allowance_eur": basic_allowance_eur(year),
                "zones": zones,
                "citation": source.citation,
                "source": source.source,
                "reference": source.reference,
            }))
        }
        Some("curve") => {
            let year = tax_year(request)?;
            let max = request["max"].as_i64().ok_or("max must be a whole number")?;
            let step = request["step"].as_i64().ok_or("step must be a whole number")?;
            if step <= 0 || max < 0 {
                return Err("step must be positive and max not negative".into());
            }
            let mut points = Vec::new();
            let mut euros = 0i64;
            while euros <= max {
                let assessment = assess(year, &unit(request, i128::from(euros))?)
                    .map_err(|e| e.to_string())?;
                points.push(json!([euros, assessment.income_tax.whole_euros()]));
                euros += step;
            }
            Ok(json!({ "year": year.year(), "points": points }))
        }
        Some("rates") => {
            let year = tax_year(request)?;
            let max = request["max"].as_i64().ok_or("max must be a whole number")?;
            let step = request["step"].as_i64().ok_or("step must be a whole number")?;
            if step <= 0 || max < 0 {
                return Err("step must be positive and max not negative".into());
            }
            let mut points = Vec::new();
            let mut euros = 0i64;
            while euros <= max {
                let explanation = explain(year, &unit(request, i128::from(euros))?)
                    .map_err(|e| e.to_string())?;
                points.push(json!([euros, explanation.average_rate, explanation.marginal_rate]));
                euros += step;
            }
            Ok(json!({ "year": year.year(), "points": points }))
        }
        Some("compare") => {
            let euros = euros(request)?;
            let rows: Vec<Value> = TaxYear::ALL
                .iter()
                .map(|year| {
                    let explanation = explain(*year, &unit(request, euros)?)
                        .map_err(|e| e.to_string())?;
                    Ok(json!({
                        "year": year.year(),
                        "basic_allowance_eur": explanation.basic_allowance_eur,
                        "income_tax_eur": explanation.income_tax_eur,
                        "average_rate": explanation.average_rate,
                    }))
                })
                .collect::<Result<_, String>>()?;
            Ok(json!({ "filing": request["filing"], "euros": euros, "rows": rows }))
        }
        _ => Err("unknown op".into()),
    }
}

fn answer(input: &[u8]) -> Value {
    let request: Value = match serde_json::from_slice(input) {
        Ok(request) => request,
        Err(error) => return json!({ "error": format!("invalid JSON: {error}") }),
    };
    respond(&request).unwrap_or_else(|error| json!({ "error": error }))
}

/// Reserves `len` bytes in the module's memory for the host to write a request into.
#[unsafe(no_mangle)]
pub extern "C" fn fiskus_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let pointer = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    pointer
}

/// Frees a buffer returned by `fiskus_alloc` or `fiskus_call`.
///
/// # Safety
///
/// `pointer` and `len` must come from a single earlier call to `fiskus_alloc` or `fiskus_call`,
/// and the buffer must not be used afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fiskus_free(pointer: *mut u8, len: usize) {
    unsafe { drop(Vec::from_raw_parts(pointer, 0, len)) };
}

/// Runs one JSON request and returns a pointer and length packed into one `u64`, high bits first.
///
/// # Safety
///
/// `pointer` and `length` must describe `length` readable bytes, for example a buffer from
/// `fiskus_alloc` that the host has written.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fiskus_call(pointer: *const u8, length: usize) -> u64 {
    let input = unsafe { slice::from_raw_parts(pointer, length) };
    let output = answer(input).to_string().into_bytes().into_boxed_slice();
    let output_length = output.len();
    let output_pointer = Box::into_raw(output) as *mut u8;
    ((output_pointer as u64) << 32) | output_length as u64
}
