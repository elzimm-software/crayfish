use std::rc::Rc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Vec3;

#[derive(Default, Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(albedo: impl Into<Color>, fuzz: f64) -> Rc<Self> {
        Rc::new(Self {
            albedo: albedo.into(),
            fuzz: fuzz.min(1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::unit_vector(Vec3::reflect(ray.direction, rec.normal)) + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::with_time(rec.p, reflected, ray.time);
        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}