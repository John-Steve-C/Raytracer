pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
};

pub trait Material: Send + Sync {
    //材料对光线的反射情况
    fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> Option<ScatterRecord> {
        None
    }
    //材料发出的光的颜色，默认是黑色
    fn emitted(&self, _r_in: Ray, _rec: HitRecord, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
    // pdf 优化，默认返回 0
    fn scattering_pdf(&self, _r_in: Ray, _rec: HitRecord, _scattered: Ray) -> f64 {
        0.
    }
}

pub struct ScatterRecord {
    //保存反射的结果
    pub attenuation: Vec3, //衰减系数
    pub scattered: Ray,    //反射光
    pub pdf: f64,
}

impl ScatterRecord {
    pub fn new(_scat: Ray, _att: Vec3, _pdf: f64) -> Self {
        Self {
            attenuation: _att,
            scattered: _scat,
            pdf: _pdf,
        }
    }
}
