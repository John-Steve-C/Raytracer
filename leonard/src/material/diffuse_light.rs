use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::Material,
    texture::Texture,
};

use super::ScatterRecord;

#[derive(Clone, Copy)]
pub struct DiffuseLight<T>
where
    T: Texture,
{
    // 会发光的金属
    pub emit: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.get_color_value(u, v, p)
    }
}
