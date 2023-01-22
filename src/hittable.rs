use crate::ray::Ray;
use crate::vector3d::{Point, Vector3D};
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    p: Point,
    t: f64,
    normal: Vector3D,
    front_face: bool
}

#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects: std::vec::Vec<Sphere>
}

impl HitRecord {
    pub fn new(point: Point, t: f64, direction:Vector3D, center: Vector3D, radius: f64) -> HitRecord {
        let normal = (point - center) / radius;
        let face = direction.dot(normal) < 0.0;
        let outward_normal = match face {
            true  => normal,
            false => -normal
        };

        HitRecord {
            p: point,
            t: t,
            front_face: face,
            normal: outward_normal,
        }
    }

    pub fn set_face_normal(&mut self, ray:Ray, outward_normal: Vector3D) {
        self.front_face = self.set_front_face(ray.direction, outward_normal);
        self.normal = self.set_normal(outward_normal);
    }

    fn set_front_face(self, direction: Vector3D, normal: Vector3D) -> bool {
        direction.dot(normal) < 0.0
    }

    fn set_normal(self, normal: Vector3D) -> Point {
        match self.front_face {
            true  => normal,
            false => -normal
        }    
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn add(mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn clear(mut self) {
        self.objects.clear();
    }
}

impl HittableT for HittableList {    
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let mut closest: f64 = max;
        let mut hit = None;

        for  object in &self.objects {            
            match object.hit(ray, min, closest) {
                Some(last_hit) => {
                    closest = last_hit.t;
                    hit = Some(last_hit);
                }
                None => todo!(),
            }                 
        }

        hit
    }
}

pub trait HittableT {
    fn hit(&self, ray:Ray, min: f64, max: f64) -> Option<HitRecord>;
}