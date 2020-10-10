mod tracer;
mod render;
use self::render::render::output_ppm;

fn main() {
    // output a sample ppm image
    output_ppm();
}
