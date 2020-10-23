use super::material;
use super::{objects, ray, vec3};
pub struct Metal {
    albedo: vec3::Vec3,
}
impl Metal {
    pub fn new(color: vec3::Vec3) -> Self {
        Self {
            albedo: color,
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
        *scattered = ray::Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        vec3::dot_vec3(&scattered.direction, &rec.normal) > 0.0
    }
}
