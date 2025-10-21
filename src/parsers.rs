pub fn parse_positive_f64(s: &str) -> Result<f64, String> {
    match s.parse::<f64>() {
        Ok(value) if value > 0.0 => Ok(value),
        Ok(value) => Err(format!("the value must be positive, but got '{}'", value)),
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}
