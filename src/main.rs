mod renderer;
mod scene;
mod tracer;
mod utils;

use self::renderer::{
    naive_renderer::NaiveRenderer, parallelism_renderer::ParellelismRenderer, renderer::Renderer,
};
use self::scene::{random_plain::RandomPlain, three_spheres_plain::ThreeSpheresPlain};
use std::sync::Arc;

fn main() {
    let scene = Arc::new(ThreeSpheresPlain::new());
    let renderer = ParellelismRenderer::new();
    renderer.render(scene);
}
