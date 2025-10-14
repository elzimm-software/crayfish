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
use crate::material::{Dielectric, Lambertian, Metal};
use crate::utils::{fPI, Vec3};

fn main() {
    let mut image = Image::from(16.0 / 9.0, 1920);
    let camera = Camera::from(
        &image,
        100,
        50,
        90.0,
        Point3::from(-2.0,2.0,1.0),
        -Point3::unit_z(),
        Vec3::unit_y()
    );

    let world = HittableList::from(vec![
        Sphere::from((0.0,-100.5,-1.0), 100.0, Lambertian::from((0.8,0.8,0.0))),
        Sphere::from((0.0, 0.0, -1.2), 0.5, Lambertian::from((0.1,0.2,0.5))),
        Sphere::from((-1.0, 0.0, -1.0), 0.5, Dielectric::from(1.50)),
        Sphere::from((-1.0, 0.0, -1.0), 0.4, Dielectric::from(1.00/1.50)),
        Sphere::from((1.0, 0.0, -1.0), 0.5, Metal::from((0.8, 0.8, 0.2), 1.0)),
    ]);

    camera.render(&mut image, &world);

    image.save("image.png");
}
