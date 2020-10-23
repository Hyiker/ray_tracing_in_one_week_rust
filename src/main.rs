mod render;
mod tracer;
mod utils;
use self::render::render::write_color;
use self::tracer::camera::Camera;
use self::tracer::objects::{
    hittable,
    materials::{lambertian::Lambertian, metal::Metal},
    sphere::Sphere,
};
use self::tracer::ray::Ray;
use self::tracer::vec3::Vec3;
use self::utils::functions::random_double;
use std::rc::Rc;
fn main() {
    // output a sample ppm image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // WORLD SCENE SETTING UP
    let mut world = hittable::HittableList::default();
    // add objects
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Some(material_ground),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Some(material_center),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

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
