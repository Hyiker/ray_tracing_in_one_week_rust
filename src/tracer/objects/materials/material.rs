use super::super::super::{ray, vec3};
use super::super::hittable::HitRecord;
pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut ray::Ray,
    ) -> bool;
}
