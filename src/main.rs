mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
pub mod camera;
pub mod material;

use sphere::Sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use std::sync::Arc;
use vec3::{Vec3};
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::color::Color;   

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    let material_glass = Arc::new(Dielectric::new(1.5));
    // World
    let mut world = HittableList::new(); 
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_glass.clone())));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_glass.clone())));

    // Camera
    let camera = Camera::new(aspect_ratio, width, samples_per_pixel);

    let file = File::create("./image.ppm")
                        .expect("Failed to create data file");
    let mut writer = BufWriter::new(file);
    let start_time = Instant::now();
    eprintln!("Rendering image...");
    camera.render(&world, &mut writer);

    writer.flush().expect("Failed to flush buffer");
    let duration = start_time.elapsed();
    eprintln!("Finished in: {:.2?}", duration);
}
