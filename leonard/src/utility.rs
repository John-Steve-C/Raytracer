use crate::{
    basic_component::vec3::Vec3
};
use rand::Rng;
use std::f64::consts::PI;

pub fn degree_to_radian(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + rng.gen::<f64>() * (max - min)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    //防止越界
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn get_pixel_color(color: Vec3, sample_per_pixel: i32) -> [u8; 3] {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;
    let scale = 1. / sample_per_pixel as f64;

    //每个像素格子，都求 sample 次颜色，然后求平均，得到RGB三元组
    // 进行 gamma 修正
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    [
        (clamp(r, 0., 0.999) * 256.).floor() as u8,
        (clamp(g, 0., 0.999) * 256.).floor() as u8,
        (clamp(b, 0., 0.999) * 256.).floor() as u8,
    ]
}
