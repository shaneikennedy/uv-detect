use std::io;

use crate::dependency::Dependency;

use log::debug;
use reqwest::blocking;
use serde_json::Value;

// TODO make this a much better http client, retries, backoff, error handling
pub fn resolve_on_index(dep: &Dependency) -> Result<Dependency, io::Error> {
    let url = format!("https://pypi.org/pypi/{}/json", dep.name());
    let response = blocking::get(url.as_str()).unwrap();
    let json: Value = response.json().unwrap();

    let version = if let Some(version) = json
        .get("info")
        .and_then(|info| info.get("version"))
        .and_then(|version| version.as_str())
    {
        version
    } else {
        ""
    };

    debug!("Found version: {} for {}", version, dep.name());
    match version {
        "" => Ok(Dependency::parse(dep.name().as_str()).unwrap()),
        _ => {
            let dep_str = format!("{}~={}", dep.name(), version);
            Ok(Dependency::parse(dep_str.as_str()).unwrap())
        }
    }
}
