mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use vec3::{Color, Point, Vec3};
use crate::hittable::HittableList;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    // Image
    let width: f64 = 400.00;
    let height: f64 = width / aspect_ratio;

    // World
    let mut world = HittableList::new(); 
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
        

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width / height);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / width;
    let pixel_delta_v = viewport_v / height;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = File::create("image.ppm").expect("Failed to create data file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", width as i32, height as i32)
        .expect("Failed to write PPM header");

    for h in 0..height as i32 {
        for w in 0..width as i32 {
            let pixel_center =
                pixel00_loc + (f64::from(w) * pixel_delta_u) + (f64::from(h) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);
            write_color(&mut writer, color);
        }
    }
    writer.flush().expect("Failed to flush buffer");
}
