use rand::Rng;
use crate::color::Color;
use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::rngs::ThreadRng;

pub trait Material{
    fn scatter(&self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian{albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, 
               scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64, // 0.0 = mirror, 1.0 = blurry
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, 
               scattered: &mut Ray, rng: &mut ThreadRng) -> bool {
        let reflected = Vec3::reflect(ray_in.direction.unit_vector(), rec.normal);
        *scattered = Ray::new(
            rec.point, 
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
        );
        *attenuation = self.albedo;
        
        // Only scatter if the reflected ray points outward from the surface.
        scattered.direction.dot(rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric{
            refraction_index: ri
        }
    }

    // Schlick's approximation for reflectivity
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 -cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, 
               attenuation: &mut Color, scattered: &mut Ray, rng: &mut ThreadRng) 
        -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
         let refractive_index = if rec.front_face {
             1.0 / self.refraction_index
         } else {
             self.refraction_index
         };
        
        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cannot_refract = refractive_index * sin_theta > 1.0;
        let direction = if cannot_refract 
            || Self::reflectance(cos_theta, refractive_index) < rng.random::<f64>() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refractive_index)
        };
        
        *scattered = Ray::new(rec.point, direction);
        true
    }
}