pub mod vec3;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use vec3::Vec3;

fn main() {
    let image_height = 256;
    let image_width  = 256;
    let image_path = Path::new("image.ppm");
    let image_file_name = image_path.display();

    let mut file = match File::create(&image_path) {
        Err(reason) => panic!("Couldn't open {}: {}", image_file_name, reason),
        Ok(file) => file,
    };

    file.write("P3\n".as_bytes())
        .expect("Failed to write PPM header");

    file.write(format!("{} {}\n255\n", image_width, image_height).as_bytes())
        .expect("Failed to initalise image dimensions");

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f32 = i as f32 / ((image_width - 1) as f32);
            let g: f32 = j as f32 / ((image_height - 1) as f32);
            let b: f32 = 0.25;
            
            let pixel = Vec3 { x: r, y: g, z: b };
            write_colour(&mut file, pixel);
        }
    }
}

fn write_colour(file: &mut File, pixel: Vec3) {
    let ir = (255.999 * pixel.x) as i32;
    let ig = (255.999 * pixel.y) as i32;
    let ib = (255.999 * pixel.z) as i32;

    file.write( format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .expect(format!("Failed to write pixel").as_str());
}
