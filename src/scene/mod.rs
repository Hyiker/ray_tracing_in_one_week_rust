pub mod random_plain;
pub mod scene;
pub mod three_spheres_plain;
use crate::tracer::objects::hittable;
use crate::tracer::{
    camera::Camera,
    objects::{
        materials::{
            dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal,
        },
        sphere::Sphere,
    },
    vec3::Vec3,
};
use crate::utils::functions::{random_double, random_double_range};
use std::sync::Arc;
