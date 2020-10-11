use crate::tracer::vec3::Vec3;
pub fn write_color(pixel_color: Vec3) {
    let multi = 255.999;
    print!(
        "{} {} {}\n",
        (multi * pixel_color.x()) as i32,
        (multi * pixel_color.y()) as i32,
        (multi * pixel_color.z()) as i32
    );
}
