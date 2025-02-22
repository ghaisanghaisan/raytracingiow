use std::f64::consts::PI;

use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn degrees_to_radians(x: f64) -> f64 {
    x * PI / 180.0
}
