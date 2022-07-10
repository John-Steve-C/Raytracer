pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
    //材料对光线的反射情况
}

pub struct ScatterRecord {
    //保存反射的结果
    pub attenuation: Vec3, //衰减系数
    pub scattered: Ray,    //反射光
}
