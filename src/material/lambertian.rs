use std::rc::Rc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Vec3;

#[derive(Default, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn from(albedo: impl Into<Color>) -> Rc<Self> {
        Rc::new(Self {
            albedo: albedo.into(),
        })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        Some((Ray::with_time(rec.p, scatter_direction, ray.time), self.albedo))
    }
}