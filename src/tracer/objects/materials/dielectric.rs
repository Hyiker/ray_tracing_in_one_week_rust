use super::material;
use super::{objects, ray, vec3};
use crate::utils::functions::{fmin, random_double};
pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl material::Material for Dielectric {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &objects::hittable::HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = vec3::Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = fmin(vec3::dot_vec3(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = refraction_ratio * sin_theta > 1.0;
        let direction: vec3::Vec3;
        if cannot_reflect || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = ray::Ray::new(&rec.p, &direction);
        true
    }
}
