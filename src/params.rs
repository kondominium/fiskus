// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
use std::sync::OnceLock;

use serde::Deserialize;

use crate::{Error, TaxYear};

#[derive(Debug, Deserialize)]
pub(crate) struct Parameter {
    pub value_eur: i128,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Parameters {
    pub basic_allowance: Parameter,
}

static PARAMETERS_2026: OnceLock<Result<Parameters, String>> = OnceLock::new();

pub(crate) fn for_year(year: TaxYear) -> Result<&'static Parameters, Error> {
    let loaded = match year {
        TaxYear::Y2026 => PARAMETERS_2026.get_or_init(|| {
            toml::from_str(include_str!("../rules/2026/parameters.toml")).map_err(|e| e.to_string())
        }),
    };
    loaded
        .as_ref()
        .map_err(|reason| Error::Parameters(reason.clone()))
}
