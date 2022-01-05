use std::ops;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    
    pub fn length(&self) -> f32 {
        self.length_squared()
            .sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        return (self.x * self.x) + 
               (self.y * self.y) + 
               (self.z * self.z);
    }

    pub fn dot(self, vec: Vec3) -> f32 {
          self.x * vec.x 
        + self.y * vec.y
        + self.z * vec.z
    }

    pub fn cross(self, vec: Vec3) -> Vec3 {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;

        Vec3 { x, y, z }
    }

    pub fn unit_vector(self) -> Vec3 {
        let length = self.length();
        self / length
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        let x = self.x + _rhs.x;
        let y = self.y + _rhs.y;
        let z = self.z + _rhs.z;

        Vec3 { x, y, z }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Self::Output {
        let x = self.x - _rhs.x;
        let y = self.y - _rhs.y;
        let z = self.z - _rhs.z;

        Vec3 { x, y, z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scale: f32) -> Self::Output {
        let x = self.x * scale;
        let y = self.y * scale;
        let z = self.z * scale;

        Vec3 { x, y, z }        
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        let x = self.x * _rhs.x;
        let y = self.y * _rhs.y;
        let z = self.z * _rhs.z;

        Vec3 { x, y, z }        
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scale: f32) {
        *self = Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;

        Vec3 { x, y, z }
    }
}