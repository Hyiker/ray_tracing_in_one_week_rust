use super::constants;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * constants::PI / 180.0
}
