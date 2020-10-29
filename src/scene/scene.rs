use super::*;
use crate::tracer::objects::hittable;
pub struct SceneConfig {
    // output a sample ppm image
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub dist_to_focus: f64,
    pub aperture: f64,
}
impl Clone for SceneConfig {
    fn clone(&self) -> Self {
        SceneConfig {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            image_height: self.image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            look_from: self.look_from,
            look_at: self.look_at,
            vup: self.vup,
            dist_to_focus: self.dist_to_focus,
            aperture: self.aperture,
        }
    }
}
pub trait Scene {
    fn get_world(&self) -> &hittable::HittableList;
    fn get_config(&self) -> &SceneConfig;
    fn get_camera(&self) -> &Camera;
}
