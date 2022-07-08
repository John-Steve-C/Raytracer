use rand::Rng;
use std::f64::consts::PI;

pub fn degree_to_radian(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + rng.gen::<f64>() * (max - min)
}
