use super::*;
use num_cpus;
use std::sync::Arc;
use std::thread;

pub struct ParellelismRenderer {
    cores: usize,
}

impl ParellelismRenderer {
    pub fn new() -> Self {
        let cores = num_cpus::get();
        ParellelismRenderer { cores: cores }
    }
}
impl Renderer for ParellelismRenderer {
    fn render(&self, scene: Arc<dyn Scene>) {
        let config = scene.get_config();
        print!("P3\n{} {}\n255\n", config.image_width, config.image_height);
        for k in 0..self.cores {
            eprint!("\rCreating renderer threads: {}", k + 1);
            thread::spawn(move || {
                // FIXME
                for j in (0..(config.image_height - (k as i32))).step_by(self.cores).rev() {
                    eprint!("\rScan lines remaining: {} ", j);
                    for i in 0..config.image_width {
                        let mut pixel_color = Vec3::default();
                        for _ in 0..config.samples_per_pixel {
                            let u =
                                (i as f64 + random_double()) / (config.image_width as f64 - 1.0);
                            let v =
                                (j as f64 + random_double()) / (config.image_height as f64 - 1.0);
                            let r = scene.get_camera().get_ray(u, v);
                            pixel_color += r.color(scene.get_world(), config.max_depth);
                        }

                        let pixel = format_pixel(pixel_color, config.samples_per_pixel as usize);
                    }
                }
            });
        }
        eprint!("\n");

        eprintln!("\nWriting into Disk...");
        eprintln!("\nDone.");
    }
}
