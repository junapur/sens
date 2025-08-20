pub fn parse_positive_f64(s: &str) -> Result<f64, String> {
    let value: f64 = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", s))?;
    if value > 0.0 {
        Ok(value)
    } else {
        Err(format!("the value must be positive, but got '{}'", value))
    }
}
