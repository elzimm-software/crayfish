mod camera;
mod color;
mod hittable;
mod image;
mod ray;
mod utils;
mod material;

use std::rc::Rc;
use hittable::HittableList;
use hittable::Sphere;
use utils::Point3;
use camera::Camera;
use image::Image;
use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::utils::{fPI, rand_f64, rand_f64_in, Vec3};

fn main() {
    let mut image = Image::from(
        16.0 / 9.0,
        400
    );
    let camera = Camera::from(
        &image,
        100,
        50,
        20.0,
        (13.0,2.0,3.0),
        Point3::zeros(),
        Vec3::unit_y(),
        0.6,
        10.0,
    );

    let mut world = HittableList::from(vec![
        Sphere::from((0.0,-1000.0, 0.0), 1000.0, Lambertian::from((0.5,0.5,0.5))),
        Sphere::from((0.0, 1.0, 0.0), 1.0, Dielectric::from(1.5)),
        Sphere::from((-4.0, 1.0, 0.0), 1.0, Lambertian::from((0.4,0.2,0.1))),
        Sphere::from((0.4,1.0,0.0), 1.0, Metal::from((0.7, 0.6, 0.5), 0.0))
    ]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3::from(a as f64 + 0.9 * rand_f64(), 0.2, b as f64 + 0.9 * rand_f64());
            if (center - Point3::from(4.0,0.2,0.0)).length() > 0.9 {
                world.add(if choose_mat < 0.8 {
                    let center2 = center + Vec3::from_y(rand_f64_in(0.0, 0.5));
                    Sphere::with_time(center, center2, 0.2, Lambertian::from(Color::random() * Color::random()))
                } else if choose_mat < 0.95 {
                    Sphere::from(center, 0.2, Metal::from(Color::random_in(0.5, 1.0), rand_f64_in(0.0, 0.5)))
                } else {
                    Sphere::from(center, 0.2, Dielectric::from(1.5))
                });
            }
        }
    }

    camera.render(&mut image, &world);

    image.save("image.png");
}
