pub mod vector3d;
pub mod ray;
pub mod hittable;
pub mod sphere;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use vector3d::Vector3D;
use vector3d::Colour;
use vector3d::Point;

use ray::Ray;
use hittable::{HitRecord, HittableT};
use sphere::Sphere;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width:f64  = 400.0;
    let image_height:f64 = 400.0 / aspect_ratio;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * aspect_ratio;
    let focal_length: f64 = 1.0;

    let origin: Vector3D = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal: Vector3D = Vector3D { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical: Vector3D = Vector3D { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner: Vector3D = origin - horizontal/2.0 - vertical/2.0 - Vector3D { x: 0.0, y: 0.0, z: focal_length};
    
    // Scene
    //let mut world = HittableList::new();
    let mut world: Vec<Sphere> = Vec::new();
    world.push( Sphere::new( Point::new(0.0,0.0,-1.0), 0.5) );
    world.push( Sphere::new( Point::new(0.0,-100.5,-1.0), 100.0) );

    // Render
    let image_path: &Path = Path::new("image.ppm");
    let image_file_name: std::path::Display<'_> = image_path.display();

    let mut file: File = match File::create(&image_path) {
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

            let direction: Vector3D = lower_left_corner
                                        + (horizontal * u) 
                                        + (vertical * v);
            
            let ray: Ray = Ray { origin, direction };
            let pixel: Vector3D = ray_colour(ray, &world);
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

fn ray_colour(ray: Ray, world: &Vec<Sphere>) -> Colour {
    let white: Vector3D = Colour { x: 1.0, y: 1.0, z: 1.0 };
    let blue: Vector3D = Colour { x:0.5, y: 0.7, z: 1.0 };

    match hit_world(world, ray, 0.0, f64::MAX) {
        Some(hit) => {
            0.5 * (hit.normal + white)
        },
        None => {
            // let t = hit_sphere(Point::new(0.0,0.0,-1.0), 0.5, &ray);
            // if t > 0.0 {
            //     let n = unit_vector(ray.at(t) - Point::new(0.0,0.0,-1.0));
            //     return 0.5 * Colour::new(n.x + 1.0,n.y + 1.0,n.z + 1.0)
            // }
            let unit_direction: Vector3D = unit_vector(ray.direction);
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
                
            ((1.0 - t) * white)  + (t * blue)
        }
    }
}

fn unit_vector(vec: Vector3D) -> Vector3D {
    let length: f64 = vec.length();
    vec / length
}

fn hit_sphere(center: Vector3D, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);    
    let c = oc.dot(oc) - (radius * radius);

    let discriminant =  (half_b * half_b) - (a*c);
    match discriminant < 0.0 {
        true => -1.0,
        false => {
            let sqrt = f64::sqrt(discriminant);
            (-half_b - sqrt) / a
        }
    }
}

/*
Determine if we hit an object in our world. 
 */
fn hit_world(world: &Vec<Sphere>, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
    let mut closest: f64 = max;
    let mut hit: Option<HitRecord> = None;

    for  object in world {            
        match object.hit(ray, min, closest) {
            Some(last_hit) => {
                //println!("hit_world:: last_hit {0}", last_hit.t);
                closest = last_hit.t;
                hit = Some(last_hit);
            }
            None => {},
        }                 
    }

    hit
}