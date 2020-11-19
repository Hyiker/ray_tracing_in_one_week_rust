pub mod naive_renderer;
pub mod renderer;
pub mod parallelism_renderer;
use crate::scene::scene::{Scene, SceneConfig};
use crate::tracer::vec3::Vec3;
use crate::utils::functions::random_double;
use renderer::{format_pixel, Renderer,ThreadScene};
use std::rc::Rc;
use std::sync::Arc;