mod render;
mod scene;
mod tracer;
mod utils;

use self::render::render::render_and_output;
use self::scene::{random_plain::RandomPlain, three_spheres_plain::ThreeSpheresPlain};
use std::rc::Rc;
fn main() {
    let scene = Rc::new(RandomPlain::new());

    render_and_output(scene);
}
