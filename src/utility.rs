use rand::random_range;

pub fn random_double() -> f64 {
    random_range(0.0..=1.0)
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    random_range(min..=max)
}
