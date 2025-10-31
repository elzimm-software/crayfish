use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::utils::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn from(center: impl Into<Point3>, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        Rc::new(Self {
            center: Ray::from(center.into(), Vec3::zeros()),
            radius: radius.max(0.0),
            material,
        })
    }

    pub fn with_time(initial_center: impl Into<Point3>, final_center: impl Into<Point3>, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        let initial_center = initial_center.into();
        Rc::new(Self {
            center: Ray::from(initial_center.clone(), final_center.into() - initial_center),
            radius: radius.max(0.0),
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(ray.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        Some(HitRecord::from(
            p,
            root,
            ray,
            self.material.clone(),
            (p - current_center) / self.radius,
        ))
    }
}