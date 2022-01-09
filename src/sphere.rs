use crate::hittable::{Hittable, HitRecord};
use crate::vector3d::Point;


pub struct Sphere {
    center: Point,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(self: HitRecord, ray:Ray, min: f64, max: f64, rec: &HitRecord) -> bool {
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);    
        let c = oc.dot(oc) - (radius * radius);

        let discriminant =  (half_b * half_b) - (a*c);
        if (discriminant < 0.0) {
            return false;
        }
        
        let sqrt = f64::sqrt(discriminant);
        let root = (-half_b - sqrt) / a;
        if (root < min) || (max < root) {
            root = (-half_b + sqrt) / a;
            if (root < min) || (max < root){
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - center) / radius;
        rec.set_face_normal(ray, outward_normal);
        
        return true;
    }
}
