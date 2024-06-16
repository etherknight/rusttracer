#![allow(unused_imports)]
#![allow(dead_code)]
use std::{f64, ops};
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Div, Mul};
use std::path::Path;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

type Color = Vec3;
type Point = Vec3;

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn dot(&self, rhs: Vec3) -> f64 {
        (self.x * rhs.x)
        + (self.y * rhs.y)
        + (self.z * rhs.z)
    }

    fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    fn unit_vector(&self) -> Vec3{
        self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z
        )
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs
        )
    }
}
impl ops::DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

struct Ray {
    origin: Point,
    direction: Vec3
}
impl Ray {
    fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> Point {
        self.origin + (t * self.direction)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    // Image
    let width: f64 = 400.00;
    let height: f64 = width / aspect_ratio;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * f64::from(width/height);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / width;
    let pixel_delta_v = viewport_v / height;

    let viewport_upper_left = camera_center
                                    - Vec3::new(0.0, 0.0, focal_length)
                                    - viewport_u/2.0
                                    - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    write_image_header(width, height);

    for h in 0..height as i32 {
        for w in 0..width as i32 {
            let pixel_center = pixel00_loc
                                        + (f64::from(w) * pixel_delta_u)
                                        + (f64::from(h) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_direction);

            let color: Color = ray_color(ray);
            write_color(color);
        }
    }
}

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - (radius * radius);
    let discriminant = (b*b) - (4.0*a*c);

    discriminant > 0.0
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)* Color::new(1.0, 1.0, 1.0)
        + a * Color::new(0.5, 0.7, 1.0)
}
fn write_image_header(width: f64, height: f64) {
    print!("P3\n{} {}\n255\n", width as i32, height as i32);
}

fn write_color(color: Color) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    print!("{} {} {}\n", ir, ig, ib);
}