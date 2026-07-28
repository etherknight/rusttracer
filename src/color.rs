use crate::vec3::Color;
use std::io::Write;

pub fn write_color(writer: &mut impl Write, color: Color) {
    let red = float_to_rgb(color.x);
    let green = float_to_rgb(color.y);
    let blue = float_to_rgb(color.z);

    writeln!(writer, "{} {} {}", red, green, blue)
        .expect("Failed to write color line to buffer");
}

fn float_to_rgb(color: f64) -> i32 {
    (255.999 * color.clamp(0.0, 0.999)) as i32
}
