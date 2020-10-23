use crate::tracer::vec3::Vec3;
use num::clamp;
pub fn write_color(pixel_color: Vec3, samples_per_pixel: usize) -> String {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();
    format!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}
