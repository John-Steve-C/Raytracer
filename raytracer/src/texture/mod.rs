use crate::basic_component::vec3::Vec3;
pub mod checker;
pub mod image;
pub mod obj;
pub mod perlin;
pub mod solid;

pub trait Texture: Send + Sync {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    //(u, v) 纹理的坐标
}
