#![warn(missing_docs)]
pub mod app;

pub fn version() -> String {
  return env!("CARGO_PKG_VERSION").to_string();
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_version() {
    use regex::Regex;
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let mat_exists = re.find(&super::version()).is_some();
    assert_eq!(
      mat_exists, true,
      "Version in correct format exists. Actual: {}. Expected {}.",
      mat_exists, true
    );
  }
}
