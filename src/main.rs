mod render;
mod scene;
mod tracer;
mod utils;

use self::render::render::render_and_output;
use self::scene::three_spheres_plain::ThreeSpheresPlain;
use std::rc::Rc;
fn main() {
    let scene = Rc::new(ThreeSpheresPlain::new());

    render_and_output(scene);
}
