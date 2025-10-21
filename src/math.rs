pub fn get_cm_360(sensitivity: f64, dpi: u32, yaw: f64) -> f64 {
    (360.0 / (sensitivity * dpi as f64 * yaw)) * 2.54
}

pub fn convert_sensitivity(
    sensitivity: f64,
    from_yaw: f64,
    to_yaw: f64,
    from_dpi: Option<u32>,
    to_dpi: Option<u32>,
) -> f64 {
    let yaw_ratio = from_yaw / to_yaw;

    let dpi_ratio = match (from_dpi, to_dpi) {
        (Some(from), Some(to)) => from as f64 / to as f64,
        _ => 1.0,
    };

    sensitivity * yaw_ratio * dpi_ratio
}
