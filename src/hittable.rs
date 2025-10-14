use std::rc::Rc;
use crate::utils::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{Point3, Vec3};

mod sphere;
mod hittable_list;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from(p: impl Into<Point3>, t: f64, ray: impl Into<Ray>, material: Rc<dyn Material>, outward_normal: impl Into<Vec3>) -> Self {
        let p = p.into();
        let outward_normal = outward_normal.into();
        let ray = ray.into();
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        Self {
            p,
            t,
            material,
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
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord>;
}

pub use sphere::Sphere;
pub use hittable_list::HittableList;