mod render;
mod tracer;
use self::render::render::write_color;
use self::tracer::ray::{ray_color, Ray};
use self::tracer::vec3::Vec3;
fn main() {
    // output a sample ppm image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width / aspect_ratio as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in (0..image_width) {
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
