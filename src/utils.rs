pub use std::f64::consts::PI as fPI;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * fPI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / fPI
}

pub fn rand_f64() -> f64 {
    rand::rng().random()
}

pub fn rand_f64_in(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}