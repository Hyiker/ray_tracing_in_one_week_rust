mod render;
mod tracer;
mod utils;
use self::render::render::write_color;
use self::tracer::camera::Camera;
use self::tracer::objects::{hittable, sphere::Sphere};
use self::tracer::ray::Ray;
use self::tracer::vec3::Vec3;
use self::utils::functions::random_double;
fn main() {
    // output a sample ppm image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut world = hittable::HittableList::default();
    // add objects
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScan lines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone");
}
