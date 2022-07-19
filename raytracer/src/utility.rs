use crate::basic_component::vec3::Vec3;
use rand::Rng;
use std::f64::consts::PI;

pub fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn min_f64(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn degree_to_radian(deg: f64) -> f64 {
    deg * PI / 180.
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + rng.gen::<f64>() * (max - min)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max + 1)
    // 左闭右开，即 [min, max + 1)
}

pub fn random_to_sphere(r : f64, distance_squared : f64) -> Vec3 {
    let r1 = random_double(0., 1.);
    let r2 = random_double(0., 1.);
    let z = 1. + r2 * ((1. - r * r / distance_squared).sqrt() - 1.);

    let phi = 2. * PI * r1;
    let x = phi.cos() * (1. - z * z).sqrt();
    let y = phi.sin() * (1. - z * z).sqrt();

    Vec3::new(x, y, z)
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

    if r.is_nan() {r = 0.};
    if g.is_nan() {g = 0.};
    if b.is_nan() {b = 0.};

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
