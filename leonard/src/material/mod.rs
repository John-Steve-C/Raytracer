pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
};

pub trait Material {
    //材料对光线的反射情况
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
    //材料发出的光的颜色，默认是黑色
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}

pub struct ScatterRecord {
    //保存反射的结果
    pub attenuation: Vec3, //衰减系数
    pub scattered: Ray,    //反射光
}
