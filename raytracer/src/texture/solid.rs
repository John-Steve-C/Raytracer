use super::Texture;
use crate::basic_component::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct SolidColor {
    pub color_value: Vec3,
}

impl Texture for SolidColor {
    fn get_color_value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}

impl SolidColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}
