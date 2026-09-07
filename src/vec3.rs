use std::ops;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
    
    pub fn random(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(
            rng.random::<f64>(),
            rng.random::<f64>(),
            rng.random::<f64>(),
        )
    }
    pub fn random_range(rng: &mut impl Rng, min: f64, max: f64) -> Vec3 {
        let range = max - min;
        Vec3::new(
            rng.random::<f64>() * range,
            rng.random::<f64>() * range,
            rng.random::<f64>() * range,
        )
    }
    
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        loop {
            let p = Vec3::random_range(rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    
    pub fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
        Vec3::random_in_unit_sphere(rng).unit_vector()
    }
    
    pub fn random_in_hemisphere(rng: &mut impl Rng, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector(rng);
        if (on_unit_sphere.dot(*normal)) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    
    // Returns true if the vector is very close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8; 
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
    
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    /**
    etai_over_etat = eta^i / eta^t
    Where eta (Greek η ) is the refractive index of a material.
    Ref: Snell's Law.
    ### 1. The Greek Letter η ("Eta")

    In physics and optics, the refractive index of a material is conventionally
    represented by the Greek letter η (eta) (or sometimes n).

    The refractive index measures how much light slows down inside that
    material:

    • Vacuum / Air: η ≈ 1.0
    • Water: η ≈ 1.33
    • Glass: η ≈ 1.5
    • Diamond: η ≈ 2.4
    ──────
    ### 2. The Subscripts: i and t

    • ηᵢ (etai): The refractive index of the medium the ray is coming from
    (incident medium).
    • ηₜ (etat): The refractive index of the medium the ray is entering
    (transmitted medium).

    Therefore, etai_over_etat is the ratio of the two refractive indices:

       ηᵢ
      ────
       ηₜ
    */
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_thea = (-uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_thea * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;

        r_out_perp + r_out_parallel
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-8;

    fn assert_vec3_near(v1: Vec3, v2: Vec3) {
        assert!(
            (v1.x - v2.x).abs() < EPSILON,
            "x mismatch: {} vs {}",
            v1.x,
            v2.x
        );
        assert!(
            (v1.y - v2.y).abs() < EPSILON,
            "y mismatch: {} vs {}",
            v1.y,
            v2.y
        );
        assert!(
            (v1.z - v2.z).abs() < EPSILON,
            "z mismatch: {} vs {}",
            v1.z,
            v2.z
        );
    }

    #[test]
    fn test_vec3_ops() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(-u, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(u + v, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(u - v, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(u * v, Vec3::new(4.0, 10.0, 18.0));
        assert_eq!(u * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * u, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(u / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec3_assign_ops() {
        let mut u = Vec3::new(1.0, 2.0, 3.0);
        u += Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(u, Vec3::new(2.0, 3.0, 4.0));

        u *= 2.0;
        assert_eq!(u, Vec3::new(4.0, 6.0, 8.0));

        u /= 2.0;
        assert_eq!(u, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_dot_product() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, -5.0, 6.0);
        assert_eq!(u.dot(v), 12.0);
    }

    #[test]
    fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(j), k);
        assert_eq!(j.cross(k), i);
        assert_eq!(k.cross(i), j);
    }

    #[test]
    fn test_length_and_unit_vector() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v.length_squared(), 25.0);
        assert_eq!(v.length(), 5.0);

        let unit = v.unit_vector();
        assert_vec3_near(unit, Vec3::new(0.0, 0.6, 0.8));
        assert!((unit.length() - 1.0).abs() < EPSILON);
    }
}
