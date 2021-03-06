use super::material;
use super::{objects, ray, vec3};
pub struct Lambertian {
    pub albedo: vec3::Vec3,
}
impl Lambertian {
    pub fn new(color: vec3::Vec3) -> Self {
        Self {
            albedo: color,
        }
    }
}
impl material::Material for Lambertian {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &objects::hittable::HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut ray::Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::Vec3::random_unit_vector();
        *scattered = ray::Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
