mod renderer;
mod scene;
mod tracer;
mod utils;

use self::renderer::{naive_renderer::NaiveRenderer, renderer::Renderer};
use self::scene::{random_plain::RandomPlain, three_spheres_plain::ThreeSpheresPlain};
use std::rc::Rc;
fn main() {
    let scene = Rc::new(ThreeSpheresPlain::new());
    let renderer = NaiveRenderer::new();
    renderer.render(scene);
}
