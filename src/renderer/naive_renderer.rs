use super::*;
pub struct NaiveRenderer {}
impl NaiveRenderer {
    pub fn new() -> Self {
        NaiveRenderer {}
    }
}
impl Renderer for NaiveRenderer {
    fn render(&self, scene: Rc<dyn Scene>) {
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

                print!(
                    "{}\n",
                    format_pixel(pixel_color, config.samples_per_pixel as usize)
                );
            }
        }
        eprintln!("\nWriting into Disk...");
        eprintln!("\nDone.");
    }
}
