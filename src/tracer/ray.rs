use super::objects::hittable;
use super::vec3::{dot_vec3, Vec3};
use crate::utils::constants;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Vec3::default(),
            direction: Vec3::default(),
        }
    }
}
impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }
    // calculate the dest of vector
    // origin + t * dir
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    // calculate the point color on a ray
    // ray color is gradient
    // color = ( 1 - t ) * start_color + t * end_color
    pub fn color(&self, world: &hittable::HittableList, depth: i32) -> Vec3 {
        let rec = &mut hittable::HitRecord::default();
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if world.hit(self, 0.001, constants::INFINITY, rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if rec.clone()
                .mat_ptr
                .expect("HitRecord's material pointer can't be None!")
                .scatter(self, rec, &mut attenuation, &mut scattered)
            {
                return attenuation * scattered.color(world, depth - 1);
            }
            // let target = rec.p + rec.normal + Vec3::random_unit_vector();
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = dot_vec3(&oc, &r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
