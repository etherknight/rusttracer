use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1{
            image_height = 1;
        }

        let camera_center = Point::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth: 50,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v
        }
    }

    pub fn render(&self, world: &dyn Hittable, writer: &mut impl Write) {
        let mut rng = rand::rng();

        writeln!(writer, "P3\n{} {}\n255", self.image_width as i32, self.image_height as i32)
            .expect("Failed to write PPM header");

        for h in 0..self.image_height as i32 {
            for w in 0..self.image_width as i32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(w, h, &mut rng);
                    pixel_color += self.ray_color(&ray, self.max_depth, world, &mut rng);
                }

                write_color(writer, pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn get_ray(&self, w: i32, h: i32, rng: &mut impl Rng) -> Ray {
        let u_offset = rng.random::<f64>()  - 0.5;
        let v_offset = rng.random::<f64>() - 0.5;

        let pixel_sample = self.pixel00_loc
            + (f64::from(w) + u_offset) * self.pixel_delta_u
            + (f64::from(h) + v_offset) * self.pixel_delta_v;

        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable, rng: &mut ThreadRng) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        
        //Object
        let mut rec = HitRecord::new();
        if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
            let mut scattered = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if let Some(mat) = &rec.mat {
                if mat.scatter(ray ,&rec, &mut attenuation, &mut scattered, rng){
                    return attenuation * self.ray_color(&scattered, depth - 1, world, rng);
                }
            }

            return Color::new(0.0, 0.0, 0.0);
        }
        // Sky / Background
        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0)
            + a * Color::new(0.5, 0.7, 1.0)
    }
}