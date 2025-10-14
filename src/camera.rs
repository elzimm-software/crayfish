use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::image::Image;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// ------- CAMERA -------------------------------------------------------------------
#[derive(Default)]
pub struct Camera {
    focal_length: f64,
    height: f64,
    width: f64,
    center: Point3,
    x: Vec3,
    y: Vec3,
    delta_x: Vec3,
    delta_y: Vec3,
    upper_left: Vec3,
    pixel00: Vec3,
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

    pub fn render(&self, image: &mut Image, world: &dyn Hittable) {
        for (y, row) in image.buffer.enumerate_rows_mut() {
            println!("\rScanlines remaining: {}          ", image.height - y);
            for (x, _, pixel) in row {
                let pixel_center =
                    self.pixel00 + (x as f64 * self.delta_x) + (y as f64 * self.delta_y);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::from(self.center, ray_direction);
                let color = Self::ray_color(ray, world);
                write_color(pixel, color)
            }
        }

        println!("\rDone                             \n");
    }

    fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(ray, Interval::from(0.0, f64::INFINITY)) {
            return 0.5 * (rec.normal + Color::ones());
        }

        let unit_direction = Vec3::unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::ones() + a * Color::from(0.5, 0.7, 1.0)
    }
}
