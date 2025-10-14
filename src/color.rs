use crate::utils::Vec3;
use image::Rgb;
use crate::utils::Interval;
use once_cell::sync::Lazy;

static INTENSITY: Lazy<Interval> = Lazy::new(|| {
    Interval::from(0.000, 0.999)
});

pub type Color = Vec3;

#[inline(always)]
fn component_to_byte(component: f64) -> u8 {
    (256.0 * INTENSITY.clamp(component)) as u8
}

#[inline(always)]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel: &mut Rgb<u8>, color: Color) {

    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    *pixel = Rgb([
        component_to_byte(r),
        component_to_byte(g),
        component_to_byte(b),
    ]);
}
