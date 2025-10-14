use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
mod lambertian;
mod metal;

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)>;
}

pub use lambertian::Lambertian;
pub use metal::Metal;