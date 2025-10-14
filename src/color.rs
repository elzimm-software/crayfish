use crate::vec3::Vec3;
use image::Rgb;
use crate::interval::Interval;
use once_cell::sync::Lazy;

static INTENSITY: Lazy<Interval> = Lazy::new(|| {
    Interval::from(0.000, 0.999)
});

pub type Color = Vec3;

fn component_to_byte(component: f64) -> u8 {
    (256.0 * INTENSITY.clamp(component)) as u8
}

pub fn write_color(pixel: &mut Rgb<u8>, color: Color) {

    *pixel = Rgb([
        component_to_byte(color.x),
        component_to_byte(color.y),
        component_to_byte(color.z),
    ]);
}
