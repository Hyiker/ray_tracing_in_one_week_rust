use super::*;
use crate::tracer::vec3::Vec3;
use num::clamp;
pub fn write_color(pixel_color: Vec3, samples_per_pixel: usize) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();
    print!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}
pub fn render_and_output(scene: Rc<dyn Scene>) {
    let config = scene.get_config();
    print!("P3\n{} {}\n255\n", config.image_width, config.image_height);
    for j in (0..config.image_height).rev() {
        eprint!("\rScan lines remaining: {} ", j);
        for i in 0..config.image_width {
            let mut pixel_color = Vec3::default();
            for _ in 0..config.samples_per_pixel {
                let u = (i as f64 + random_double()) / (config.image_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (config.image_height as f64 - 1.0);
                let r = scene.get_camera().get_ray(u, v);
                pixel_color += r.color(scene.get_world(), config.max_depth);
            }

            write_color(pixel_color, config.samples_per_pixel as usize);
        }
    }
    eprintln!("\nWriting into Disk...");
    eprintln!("\nDone.");
}
