use crate::vec3::{Vec3};
use std::io::Write;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    
    0.0
}
pub fn write_color(writer: &mut impl Write, color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = linear_to_gamma(scale * color.x);
    let g = linear_to_gamma(scale * color.y);
    let b = linear_to_gamma(scale * color.z);

    let red = float_to_rgb(r);
    let green = float_to_rgb(g);
    let blue = float_to_rgb(b);

    writeln!(writer, "{} {} {}", red, green, blue)
        .expect("Failed to write color line to buffer");
}

fn float_to_rgb(color: f64) -> i32 {
    (255.999 * color.clamp(0.0, 0.999)) as i32
}
