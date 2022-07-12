use crate::basic_component::vec3::Vec3;
pub mod checker;
pub mod solid;

pub trait Texture {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
