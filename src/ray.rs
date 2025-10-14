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

    pub fn from(origin: impl Into<Point3>, direction: impl Into<Vec3>) -> Self {
        Self {
            origin:origin.into(),
            direction:direction.into(),
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
