use super::Vec3;
struct Ray {
    origin: Vec3,
    direction: Vec3,
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
    fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Ray {
            origin: *origin,
            direction: *direction,
        }
    }
    // calculate the dest of vector
    // origin + t * dir
    fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

// calculate the point color on a ray
// ray color is gradient
fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
