mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod interval;

use std::rc::Rc;
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use camera::Camera;
use image::Image;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::ones());
    }

    let unit_direction = Vec3::unit_vector(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::ones() + a * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    let mut image = Image::from(16.0 / 9.0, 1920);
    let camera = Camera::from(&image, 1.0, 2.0);

    let world = HittableList::from(vec![
        Rc::new(Sphere::from(-Point3::unit_z(), 0.5)),
        Rc::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0))
    ]);

    for (y, row) in image.buffer.enumerate_rows_mut() {
        println!("\rScanlines remaining: {}          ", image.height - y);
        for (x, _, pixel) in row {
            let pixel_center =
                camera.pixel00 + (x as f64 * camera.delta_x) + (y as f64 * camera.delta_y);
            let ray_direction = pixel_center - camera.center;
            let ray = Ray::from(camera.center, ray_direction);
            let color = ray_color(ray, &world);
            write_color(pixel, color);
        }
    }

    image.save("image.png");

    println!("\rDone                             \n");
}
