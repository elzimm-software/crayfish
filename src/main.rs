mod camera;
mod color;
mod hittable;
mod image;
mod ray;
mod utils;
mod material;

use hittable::HittableList;
use hittable::Sphere;
use utils::Point3;
use camera::Camera;
use image::Image;
use crate::color::Color;
use crate::material::{Lambertian, Metal};

fn main() {
    let mut image = Image::from(16.0 / 9.0, 1920);
    let camera = Camera::from(&image, 1.0, 2.0, 100, 50);

    let world = HittableList::from(vec![
        Rc::new(Sphere::from(-Point3::unit_z(), 0.5)),
        Rc::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0)),
    ]);

    camera.render(&mut image, &world);

    image.save("image.png");
}
