pub mod vec3;
pub mod ray;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use vec3::Vec3;
use vec3::Colour;
use vec3::Point;

use ray::Ray;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width  = 400;
    let image_height = 400 / (aspect_ratio as i32);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length: f32 = 1.0;

    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length};

    // Render
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
            let u: f32 = i as f32 / ((image_width - 1) as f32);
            let v: f32 = j as f32 / ((image_height - 1) as f32);

            let direction = lower_left_corner + (horizontal * u) + (vertical * v) - origin;
            
            let ray = Ray { origin, direction };
            let pixel = ray_colour(ray);
            write_colour(&mut file, pixel);
        }
    }
}

fn write_colour(file: &mut File, pixel: Colour) {
    let ir = (255.999 * pixel.x) as i32;
    let ig = (255.999 * pixel.y) as i32;
    let ib = (255.999 * pixel.z) as i32;

    file.write( format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .expect(format!("Failed to write pixel").as_str());
}

fn ray_colour(ray: Ray) -> Colour {
    let unit_direction: Vec3 = unit_vector(ray.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    let white = Colour { x: 1.0, y: 1.0, z: 1.0 };
    let blue = Colour { x:0.5, y: 0.7, z: 1.0 };

    (white * (1.0-t))  + (blue * t)
}

fn unit_vector(vec: Vec3) -> Vec3 {
    let length = vec.length();
    vec / length
}
