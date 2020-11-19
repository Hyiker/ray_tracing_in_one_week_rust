use super::*;
use super::{
    hittable,
    scene::{Scene, SceneConfig},
};
pub struct ThreeSpheresPlain {
    world: hittable::HittableList,
    config: SceneConfig,
    camera: Camera,
}
impl ThreeSpheresPlain {
    pub fn new() -> Self {
        let image_width = 720;
        let aspect_ratio = 16.0 / 9.0;
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let look_from = Vec3::new(3.0, 3.0, 2.0);

        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = (look_from - look_at).length();
        let aperture = 2.0;
        let config = SceneConfig {
            // output a sample ppm image
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: (image_width as f64 / aspect_ratio) as i32,
            samples_per_pixel: 100,
            max_depth: 50,
            look_from: look_from,
            look_at: look_at,
            vup: vup,
            dist_to_focus: dist_to_focus,
            aperture: aperture,
        };
        // WORLD SCENE SETTING UP
        let mut world = hittable::HittableList::default();
        // add objects
        let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
        let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
        let material_left = Arc::new(Dielectric::new(1.5));
        let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Some(material_ground),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Some(material_center),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Some(material_left.clone()),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.4,
            Some(material_left.clone()),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Some(material_right),
        )));
        ThreeSpheresPlain {
            world: world,
            config: config,
            camera: Camera::new(
                look_from,
                look_at,
                vup,
                20.0,
                aspect_ratio,
                aperture,
                dist_to_focus,
            ),
        }
    }
}
impl Scene for ThreeSpheresPlain {
    fn get_world(&self) -> &hittable::HittableList {
        return &self.world;
    }
    fn get_config(&self) -> &SceneConfig {
        &self.config
    }
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
}
