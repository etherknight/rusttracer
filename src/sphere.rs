use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use crate::material::Material;

use std::sync::Arc;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        
        Sphere { 
            center, 
            radius, 
            mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let origin_center: Vec3 = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(origin_center);
        let c = origin_center.length_squared() - (self.radius * self.radius);
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrt_discriminant) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_front_face_normal(ray, outward_normal);
        hit_record.mat = Some(self.mat.clone());

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::material::Lambertian;
    use super::*;
    #[test]
    fn test_sphere_hit() {
        let material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material);
        let r = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let mut h = HitRecord::new();

        assert!(s.hit(&r, 0.001, f64::MAX, &mut h));
    }

    #[test]
    fn test_sphere_miss(){
        let material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material);
        let r = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let mut h = HitRecord::new();

        assert!(!s.hit(&r, 0.001, 0.0, &mut h));
    }
}