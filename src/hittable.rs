use crate::ray::Ray;
use crate::vector3d::{Point, Vector3D};

pub trait HittableT {
    fn hit(&self, ray:Ray, min: f64, max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Point,
    pub t: f64,
    pub normal: Vector3D,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Point, t: f64, direction:Vector3D, center: Vector3D, radius: f64) -> HitRecord {
        let point_normal = (point - center) / radius;
        let front_face = HitRecord::set_front_face_impl(direction, point_normal);
        let normal = HitRecord::set_normal_impl(front_face, point_normal);

        HitRecord { point, t, front_face, normal }
    }

    pub fn set_face_normal(&mut self, ray:Ray, outward_normal: Vector3D) {
        self.front_face = self.set_front_face(ray.direction, outward_normal);
        self.normal = self.set_normal(outward_normal);
    }

    pub fn set_front_face(self, direction: Vector3D, normal: Vector3D) -> bool {
        HitRecord::set_front_face_impl(direction, normal)
    }

    fn set_front_face_impl(direction: Vector3D, normal: Vector3D) -> bool {
        direction.dot(normal) < 0.0
    }

    pub fn set_normal(self, normal: Vector3D) -> Point {
        HitRecord::set_normal_impl(self.front_face, normal)    
    }

    fn set_normal_impl(front_face: bool, normal: Vector3D) -> Point {
        match front_face {
            true  => normal,
            false => -normal
        }
    }
    
}