pub fn get_cm_360(sensitvity: f64, dpi: u32, yaw: f64) -> f64 {
    (360.0 / (sensitvity * dpi as f64 * yaw)) * 2.54
}

pub fn convert_sensitivity(
    sensitivity: f64,
    from_yaw: f64,
    to_yaw: f64,
    from_dpi: Option<u32>,
    to_dpi: Option<u32>,
) -> f64 {
    let base_conversion = sensitivity * from_yaw / to_yaw;

    match (from_dpi, to_dpi) {
        (Some(from), Some(to)) => base_conversion * (from as f64) / (to as f64),
        _ => base_conversion,
    }
}
