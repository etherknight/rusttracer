
use crate::vector3d::{Point, Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3D
}

impl Ray {
    pub fn at(self, t: f64) -> Point {
        self.origin + (self.direction * t)
    }
}