use super::*;
use super::{
    hittable,
    scene::{Scene, SceneConfig},
};
use std::rc::Rc;
pub struct RandomPlain {
    world: hittable::HittableList,
    config: SceneConfig,
    camera: Camera,
}
impl RandomPlain {
    fn random_scene() -> hittable::HittableList {
        let mut world = hittable::HittableList::default();
        let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        world.add(Rc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Some(ground_material),
        )));
        for a in -11..12 {
            for b in -11..12 {
                let choose_mat = random_double();
                let center = Vec3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere_material: Rc<dyn Material>;
                    if choose_mat < 0.8 {
                        let albedo = Vec3::random() * Vec3::random();
                        sphere_material = Rc::new(Lambertian::new(albedo));
                        world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::random_range(0.5, 1.0);
                        let fuzz = random_double_range(0.0, 0.5);
                        sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                    } else {
                        sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Rc::new(Sphere::new(center, 0.2, Some(sphere_material))));
                    }
                }
            }
        }
        let material1 = Rc::new(Dielectric::new(1.5));
        world.add(Rc::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Some(material1),
        )));

        let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
        world.add(Rc::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Some(material2),
        )));

        let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
        world.add(Rc::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Some(material3),
        )));
        world
    }

    pub fn new() -> Self {
        let aspect_ratio = 3.0 / 2.0;
        let image_width = 1200;
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let samples_per_pixel = 500;
        let max_depth = 50;
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;
        let config = SceneConfig {
            // output a sample ppm image
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            look_from: look_from,
            look_at: look_at,
            vup: vup,
            dist_to_focus: dist_to_focus,
            aperture: aperture,
        };
        // WORLD SCENE SETTING UP
        let world = Self::random_scene();
        RandomPlain {
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
impl Scene for RandomPlain {
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
