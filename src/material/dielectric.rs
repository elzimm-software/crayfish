use std::rc::Rc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{rand_f64, Vec3};

#[derive(Default, Copy, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self {
            refraction_index
        })
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.front_face {1.0/self.refraction_index} else {self.refraction_index};
        let unit_direction = Vec3::unit_vector(ray.direction);
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rand_f64() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        Some((Ray::with_time(rec.p, direction, ray.time), Color::ones()))
    }
}