pub use std::f64::consts::PI as fPI;
use rand::Rng;

#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * fPI / 180.0
}

#[inline(always)]
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / fPI
}

#[inline(always)]
pub fn rand_f64() -> f64 {
    rand::rng().random()
}

#[inline(always)]
pub fn rand_f64_in(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}