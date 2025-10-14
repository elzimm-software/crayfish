use crate::vec3::Vec3;
use image::Rgb;

pub type Color = Vec3;

pub fn write_color(pixel: &mut Rgb<u8>, color: Color) {
    *pixel = Rgb([
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8,
    ]);
}
