pub fn get_cm_360(sensitvity: f64, dpi: u32, yaw: f64) -> f64 {
    (360.0 / (sensitvity * dpi as f64 * yaw)) * 2.54
}

pub fn convert_sensitivity(sensitivity: f64, from_yaw: f64, to_yaw: f64) -> f64 {
    sensitivity * from_yaw / to_yaw
}

pub fn convert_sensitivity_dpi(
    sensitivity: f64,
    from_yaw: f64,
    to_yaw: f64,
    from_dpi: u32,
    to_dpi: u32,
) -> f64 {
    sensitivity * from_yaw / to_yaw * (from_dpi as f64) / (to_dpi as f64)
}
