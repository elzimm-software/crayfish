use crate::utils::{Point3, Vec3};

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
