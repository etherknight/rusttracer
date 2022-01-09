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
    let image_width:f64  = 400.0;
    let image_height:f64 = 400.0 / aspect_ratio;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length: f64 = 1.0;

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

    for j in 0..image_height as i64 {
        for i in 0..image_width as i64 {
            let u: f64 = i as f64 / ((image_width - 1.0) as f64);
            let v: f64 = j as f64 / ((image_height - 1.0) as f64);

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
    let t = hit_sphere(Point::new(0.0,0.0,-1.0), 0.5, &ray);
    if t > 0.0 {
        //println!("HIT! t={}", t);
        let n = unit_vector(ray.at(t) - Point::new(0.0,0.0,-1.0));
        return 0.5 * Colour::new(n.x + 1.0,n.y + 1.0,n.z + 1.0)
    }
    let unit_direction: Vec3 = unit_vector(ray.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    let white = Colour { x: 1.0, y: 1.0, z: 1.0 };
    let blue = Colour { x:0.5, y: 0.7, z: 1.0 };

    ((1.0-t) * white)  + (t * blue)
}

fn unit_vector(vec: Vec3) -> Vec3 {
    let length = vec.length();
    vec / length
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - (radius * radius);

    let discriminant =  (b * b) - (4.0*a*c);
    match discriminant < 0.0 {
        true => -1.0,
        false => {
            let sqrt = f64::sqrt(discriminant);
            (-b - sqrt) / (2.0 * a)
        }
    }
}