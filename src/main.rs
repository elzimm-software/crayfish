mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use camera::Camera;
use image::Image;

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let h = Vec3::dot(ray.direction, oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(-Point3::unit_z(), 0.5, ray);
    if t > 0.0 {
        #[allow(non_snake_case)]
        let N = Vec3::unit_vector(ray.at(t) - -Vec3::unit_z());
        return 0.5 * (N + Vec3::ones());
    }

    let unit_direction = Vec3::unit_vector(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::ones() + a * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    let mut image = Image::from(16.0 / 9.0, 400);
    let camera = Camera::from(&image, 1.0, 2.0);

    for (y, row) in image.buffer.enumerate_rows_mut() {
        println!("\rScanlines remaining: {}          ", image.height - y);
        for (x, _, pixel) in row {
            let pixel_center =
                camera.pixel00 + (x as f64 * camera.delta_x) + (y as f64 * camera.delta_y);
            let ray_direction = pixel_center - camera.center;
            let ray = Ray::from(camera.center, ray_direction);
            let color = ray_color(ray);
            write_color(pixel, color);
        }
    }

    image.save("image.png");

    println!("\rDone                             \n");
}
