use crate::hittable::{HittableT, HitRecord};
use crate::vector3d::Point;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl HittableT for Sphere {
    fn hit(&self, ray:Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);    
        let c = oc.dot(oc) - (self.radius * self.radius);

        let discriminant =  (half_b * half_b) - (a*c);
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt) / a;
        if (root < min) || (max < root) {
            root = (-half_b + sqrt) / a;
            if (root < min) || (max < root){
                return None;
            }
        }
        let rec = HitRecord::new(ray.at(root), 
                                root, 
                                  ray.direction, 
                                self.center, 
                                self.radius);

        
        return Some(rec);
    }
}
