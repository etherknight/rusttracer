use crate::ray::Ray;
use crate::vec3::{Point, Vector3D};

pub struct HitRecord {
    p: Point,
    normal: Vector3D,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(self, ray:Ray, outward_normal: Vector3D) {
        self.front_face = ray.direction.dot(outward_normal) < 0;
        normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
            _ => panic!("Invalid front face!")
        };
    }
}

trait Hittable {
    fn hit(self: HitRecord, ray:Ray, min: f64, max: f64, rec: &HitRecord) -> bool;
}