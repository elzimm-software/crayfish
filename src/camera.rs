use crate::image::Image;
use crate::vec3::{Point3, Vec3};

// ------- CAMERA -------------------------------------------------------------------
#[derive(Default)]
pub struct Camera {
    pub focal_length: f64,
    pub height: f64,
    pub width: f64,
    pub center: Point3,
    pub x: Vec3,
    pub y: Vec3,
    pub delta_x: Vec3,
    pub delta_y: Vec3,
    pub upper_left: Vec3,
    pub pixel00: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(image: &Image, focal_length: f64, height: f64) -> Self {
        let width = height * (image.width as f64 / image.height as f64);
        let center = Point3::zeros();
        let x = Vec3::from_x(width);
        let y = Vec3::from_y(-height);
        let delta_x = x / image.width as f64;
        let delta_y = y / image.height as f64;
        let upper_left = center - Vec3::from_z(focal_length) - x / 2.0 - y / 2.0;
        let pixel00 = upper_left + 0.5 * (delta_x + delta_y);
        Self {
            focal_length,
            height,
            width,
            center,
            x,
            y,
            delta_x,
            delta_y,
            upper_left,
            pixel00,
        }
    }
}