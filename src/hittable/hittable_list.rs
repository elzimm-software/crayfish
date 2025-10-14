use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;
use crate::utils::Interval;

#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self {
            objects
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t: Interval) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        let mut closest_so_far = t.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, (t.min, closest_so_far).into()) {
                closest_so_far = rec.t;
                best_hit = Some(rec);
            }
        }

        best_hit
    }
}
