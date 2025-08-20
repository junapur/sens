pub fn get_cm_360(sensitvity: f64, dpi: u32, yaw: f64) -> f64 {
    (360.0 / (sensitvity * dpi as f64 * yaw)) * 2.54
}

pub fn convert_sensitivity(sensitvity: f64, from_yaw: f64, to_yaw: f64) -> f64 {
    sensitvity * from_yaw / to_yaw
}
