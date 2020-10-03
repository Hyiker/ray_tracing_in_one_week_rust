use ray_tracing_in_one_week::{color, write_color};
fn output_ppm() {
    let image_width = 256;
    let image_height = 256;
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\r Scan lines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.25,
            );
            write_color(pixel_color);
        }
    }
    eprint!("\nDone.\n");
}

fn main() {
    output_ppm();
}
