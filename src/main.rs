mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
pub mod camera;

use sphere::Sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use vec3::{Vec3};
use crate::hittable::HittableList;
use crate::camera::Camera;


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new(); 
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
