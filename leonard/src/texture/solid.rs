use crate::basic_component::vec3::Vec3;

use super::Texture;
pub struct SolidColor {
    pub color_value: Vec3,
}

impl Texture for SolidColor {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
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
