use super::constants;
use rand::Rng;
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * constants::PI / 180.0
}
pub fn fmin(x: f64, y: f64) -> f64 {
    return if x < y { x } else { y };
}
