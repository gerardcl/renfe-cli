use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use std::time::Duration;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Registry {
  CratesIo,
  PyPi,
}

impl Registry {
  fn name(self) -> &'static str {
    match self {
      Self::CratesIo => "crates.io",
      Self::PyPi => "PyPI",
    }
  }

  fn url(self) -> String {
    match self {
      Self::CratesIo => format!("https://crates.io/api/v1/crates/{PACKAGE_NAME}"),
      Self::PyPi => format!("https://pypi.org/pypi/{PACKAGE_NAME}/json"),
    }
  }
}

#[derive(Deserialize)]
struct CratesIoResponse {
  #[serde(rename = "crate")]
  package: CratesIoPackage,
}

#[derive(Deserialize)]
struct CratesIoPackage {
  newest_version: String,
}

#[derive(Deserialize)]
struct PyPiResponse {
  info: PyPiPackage,
}

#[derive(Deserialize)]
struct PyPiPackage {
  version: String,
}

pub(crate) fn print_version_status(registry: Registry) {
  println!("Running renfe-cli {PACKAGE_VERSION}");

  match latest_version(registry) {
    Ok(latest) => print_comparison(registry, PACKAGE_VERSION, &latest),
    Err(error) => eprintln!(
      "Could not check {} for a newer renfe-cli version: {error}",
      registry.name()
    ),
  }
}

fn latest_version(registry: Registry) -> Result<String, String> {
  let client = Client::builder()
    .timeout(Duration::from_secs(3))
    .user_agent(format!("{PACKAGE_NAME}/{PACKAGE_VERSION}"))
    .build()
    .map_err(|error| error.to_string())?;
  let response = client
    .get(registry.url())
    .send()
    .and_then(|response| response.error_for_status())
    .map_err(|error| error.to_string())?;

  match registry {
    Registry::CratesIo => response
      .json::<CratesIoResponse>()
      .map(|response| response.package.newest_version)
      .map_err(|error| error.to_string()),
    Registry::PyPi => response
      .json::<PyPiResponse>()
      .map(|response| response.info.version)
      .map_err(|error| error.to_string()),
  }
}

fn print_comparison(registry: Registry, current: &str, latest: &str) {
  let current_version = Version::parse(current);
  let latest_version = Version::parse(latest);

  match (current_version, latest_version) {
    (Ok(current_version), Ok(latest_version)) if latest_version > current_version => println!(
      "A newer renfe-cli version is available on {}: {latest} (running {current}).",
      registry.name()
    ),
    (Ok(_), Ok(_)) => println!("renfe-cli is up to date on {}.", registry.name()),
    _ => eprintln!(
      "Could not compare the running version with version {latest} from {}.",
      registry.name()
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn registry_urls_use_the_package_name() {
    assert_eq!(
      Registry::CratesIo.url(),
      "https://crates.io/api/v1/crates/renfe-cli"
    );
    assert_eq!(Registry::PyPi.url(), "https://pypi.org/pypi/renfe-cli/json");
  }

  #[test]
  fn parses_registry_responses() {
    let crates_io: CratesIoResponse =
      serde_json::from_str(r#"{"crate":{"newest_version":"6.3.0"}}"#).unwrap();
    let pypi: PyPiResponse = serde_json::from_str(r#"{"info":{"version":"6.3.0"}}"#).unwrap();

    assert_eq!(crates_io.package.newest_version, "6.3.0");
    assert_eq!(pypi.info.version, "6.3.0");
  }

  #[test]
  fn semantic_versions_detect_updates() {
    assert!(Version::parse("6.3.0").unwrap() > Version::parse("6.2.0").unwrap());
    assert!(Version::parse("6.2.0").unwrap() <= Version::parse("6.2.0").unwrap());
  }
}
