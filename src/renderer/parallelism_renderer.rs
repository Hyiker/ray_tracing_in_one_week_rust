use super::*;
use num_cpus;
use std::sync::Arc;
use std::thread;
pub struct ParellelismRenderer<'a> {
    cores: usize,
    scene:  Arc<&'a ThreadScene>,
}

impl<'a> ParellelismRenderer<'a> {
    pub fn new(scene: Arc<&'a ThreadScene>) -> Self {
        let cores = num_cpus::get();
        ParellelismRenderer {
            cores: cores,
            scene: scene,
        }
    }
    pub fn render_line(&self, i: u32) -> String {
        let mut line = String::new();
        for j in 0..self.scene.get_config().image_width {
            line += &self.render_pixel(i, j as u32);
        }
        line
    }
    fn render_pixel(&self, i: u32, j: u32) -> String {
        let config = self.scene.get_config();
        let mut pixel_color = Vec3::default();
        for _ in 0..config.samples_per_pixel {
            let u = (i as f64 + random_double()) / (config.image_width as f64 - 1.0);
            let v = (j as f64 + random_double()) / (config.image_height as f64 - 1.0);
            let r = self.scene.get_camera().get_ray(u, v);
            pixel_color += r.color(self.scene.get_world(), config.max_depth);
        }
        format_pixel(pixel_color, config.samples_per_pixel as usize)
    }
}

impl<'a> Renderer for ParellelismRenderer<'a> {
    fn render(&self, scene: Arc<ThreadScene>) {
        let config = scene.get_config();
        print!("P3\n{} {}\n255\n", config.image_width, config.image_height);
        for k in 0..self.cores {
            eprint!("\rCreating renderer threads: {}", k + 1);
            thread::spawn(move || {
                self.render_line(1);
            });
        }
        eprint!("\n");

        eprintln!("\nWriting into Disk...");
        eprintln!("\nDone.");
    }
}
