use super::super::{ray, vec3};
pub struct HitRecord {
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot_vec3(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: vec3::Vec3::default(),
            normal: vec3::Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}
impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
        }
    }
}
impl Copy for HitRecord {}
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}
impl HittableList {
    pub fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.as_ref().hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = *temp_rec;
            }
        }
        hit_anything
    }
}
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
