pub use std::f64::consts::PI as fPI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * fPI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / fPI
}
