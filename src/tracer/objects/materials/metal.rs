use super::material;
use super::{objects, ray, vec3};
pub struct Metal {
    albedo: vec3::Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(color: vec3::Vec3, f: f64) -> Self {
        Self {
            albedo: color,
            fuzz: (if f < 1.0 { f } else { 1.0 }),
        }
    }
}
impl material::Material for Metal {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &objects::hittable::HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vec3::reflect(&r_in.direction.unit_vector(), &rec.normal);
        *scattered = ray::Ray::new(
            &rec.p,
            &(reflected + self.fuzz * vec3::Vec3::random_in_unit_sphere()),
        );
        *attenuation = self.albedo;
        vec3::dot_vec3(&scattered.direction, &rec.normal) > 0.0
    }
}
