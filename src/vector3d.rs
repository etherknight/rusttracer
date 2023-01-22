use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Colour = Vector3D;
pub type Point = Vector3D;

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.length_squared()
            .sqrt()
    }

    pub fn length_squared(self) -> f64 {
        return (self.x * self.x) + 
               (self.y * self.y) + 
               (self.z * self.z);
    }

    pub fn dot(self, vec: Vector3D) -> f64 {
          self.x * vec.x 
        + self.y * vec.y
        + self.z * vec.z
    }

    pub fn cross(self, vec: Vector3D) -> Vector3D {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;

        Vector3D::new(x, y, z)
    }

    pub fn unit_vector(self) -> Vector3D {
        let length = self.length();
        self / length
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, _rhs: Vector3D) -> Self::Output {
        let x = self.x + _rhs.x;
        let y = self.y + _rhs.y;
        let z = self.z + _rhs.z;

        Vector3D::new(x, y, z)
    }
}

impl ops::AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, _rhs: Vector3D) -> Self::Output {
        let x = self.x - _rhs.x;
        let y = self.y - _rhs.y;
        let z = self.z - _rhs.z;

        Vector3D { x, y, z }
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scale: f64) -> Self::Output {
        let x = self.x * scale;
        let y = self.y * scale;
        let z = self.z * scale;

        Vector3D { x, y, z }        
    }
}

impl ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, vec: Vector3D) -> Self::Output {
        let x = vec.x * self;
        let y = vec.y * self;
        let z = vec.z * self;

        Vector3D { x, y, z }        
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, _rhs: Vector3D) -> Self::Output {
        let x = self.x * _rhs.x;
        let y = self.y * _rhs.y;
        let z = self.z * _rhs.z;

        Vector3D { x, y, z }        
    }
}

impl ops::MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, scale: f64) {
        *self = Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }
}

impl ops::Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;

        Vector3D { x, y, z }
    }
}