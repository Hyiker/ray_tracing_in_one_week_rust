use super::super::{ray, vec3};
use super::hittable::{HitRecord, Hittable};
use super::materials::material::Material;
use std::sync::Arc;
pub struct Sphere {
    center: vec3::Vec3,
    radius: f64,
    mat_ptr: Option<Arc<dyn Material>>,
}
impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: vec3::Vec3::default(),
            radius: 0.0,
            mat_ptr: None,
        }
    }
}
impl Sphere {
    pub fn new(cen: vec3::Vec3, r: f64, mat_ptr: Option<Arc<dyn Material>>) -> Self {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: mat_ptr,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = vec3::dot_vec3(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }
}
