use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(p: Point3, t: f64, ray: Ray, outward_normal: Vec3) -> Self {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        Self {
            p,
            t,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
