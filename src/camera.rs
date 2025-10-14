use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::image::Image;
use crate::utils::{degrees_to_radians, Interval};
use crate::ray::Ray;
use crate::utils::rand_f64;
use crate::utils::{Point3, Vec3};

#[derive(Default, Copy, Clone)]
struct Basis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

#[derive(Default, Copy, Clone)]
pub struct Camera {
    focal_length: f64,
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: u32,
    vertical_fov: f64,
    center: Point3,
    delta_x: Vec3,
    delta_y: Vec3,
    upper_left: Vec3,
    pixel00: Vec3,
    basis: Basis,
    look_from: Point3,
    look_at: Point3,
    up: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(
        image: &Image,
        samples_per_pixel: u32, 
        max_depth: u32, 
        vertical_fov: f64, 
        look_from: Point3, 
        look_at: Point3, 
        up: Vec3,
    ) -> Self {
        let focal_length = (look_from - look_at).length();
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta/2.0).tan();
        let height = 2.0 * h * focal_length;
        let width = height * (image.width as f64 / image.height as f64);
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(up, w));
        let v = Vec3::cross(w, u);
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;
        let center = look_from;
        let x = width * u;
        let y = height * -v;
        let delta_x = x / image.width as f64;
        let delta_y = y / image.height as f64;
        let upper_left = center - (focal_length * w) - x/2.0 - y/2.0;
        let pixel00 = upper_left + 0.5 * (delta_x + delta_y);
        Self {
            focal_length,
            samples_per_pixel,
            pixel_sample_scale,
            max_depth,
            vertical_fov,
            center,
            delta_x,
            delta_y,
            upper_left,
            pixel00,
            basis: Basis {
                w,
                u,
                v,
            },
            look_from,
            look_at,
            up,
        }
    }

    pub fn render(&self, image: &mut Image, world: &dyn Hittable) {
        for (y, row) in image.buffer.enumerate_rows_mut() {
            println!("\rScanlines remaining: {}          ", image.height - y);
            for (x, _, pixel) in row {
                let mut color = Color::zeros();
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x,y);
                    color += Self::ray_color(ray, self.max_depth, world);
                }
                write_color(pixel, self.pixel_sample_scale * color);
            }
        }

        println!("\rDone                             \n");
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Self::sample_square();
        let sample = self.pixel00 + ((x as f64 + offset.x) * self.delta_x) + ((y as f64 + offset.y) * self.delta_y);

        let origin = self.center;
        let direction = sample - origin;

        Ray::from(origin, direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::from(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
    }

    fn ray_color(ray: Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::zeros();
        }

        if let Some(rec) = world.hit(ray, Interval::from(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.clone().material.scatter(ray, rec) {
                return attenuation * Self::ray_color(scattered, depth-1, world);
            }
            return Color::zeros();
        }

        let unit_direction = Vec3::unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::ones() + a * Color::from(0.5, 0.7, 1.0)
    }
}
