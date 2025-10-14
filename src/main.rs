mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use camera::Camera;
use image::Image;
use std::rc::Rc;

fn main() {
    let mut image = Image::from(16.0 / 9.0, 400);
    let camera = Camera::from(&image, 1.0, 2.0, 100);

    let world = HittableList::from(vec![
        Rc::new(Sphere::from(-Point3::unit_z(), 0.5)),
        Rc::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0)),
    ]);

    camera.render(&mut image, &world);

    image.save("image.png");
}
