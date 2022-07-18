use crate::{
    basic_component::vec3::Vec3,
    // hittable::HitRecord,
    material::Material,
    texture::{solid::SolidColor, Texture},
};

// use super::ScatterRecord;

#[derive(Clone, Copy)]
pub struct DiffuseLight<T>
where
    T: Texture,
{
    // 会发光的金属
    pub emit: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    // fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> Option<ScatterRecord> {
    //     None
    // }
    // 没有特殊的scatter，会自动按照 Material 中的默认函数实现
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.get_color_value(u, v, p)
    }
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(c: T) -> Self {
        Self { emit: c }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_from_color(c: Vec3) -> Self {
        Self {
            emit: SolidColor { color_value: c },
        }
    }
}
