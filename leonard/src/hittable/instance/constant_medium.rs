use std::f64::consts::E;
use std::f64::INFINITY;

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    material::{isotropic::Isotropic, Material},
    optimization::aabb::AABB,
    texture::{solid::SolidColor, Texture},
    utility::random_double,
};

pub struct ConstantMedium<TA, TB>
where
    TA: Hittable,
    TB: Material,
{
    pub boundary: TA,
    pub phase_function: TB,
    pub neg_inv_density: f64,
}

impl<TA: Hittable, TB: Material> Hittable for ConstantMedium<TA, TB> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1;
        let mut rec2;
        if let Some(rec) = self.boundary.hit(r, -INFINITY, INFINITY) {
            rec1 = rec;
        } else {
            return None;
        }
        if let Some(rec) = self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
            rec2 = rec;
        } else {
            return None;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.dir.length();
        let dis_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_dis = self.neg_inv_density * random_double(0., 1.).log(E);

        if hit_dis > dis_inside_boundary {
            return None;
        }

        let _t = rec1.t + hit_dis / ray_length;
        let _p = r.at(_t);

        Some(HitRecord {
            t: _t,
            p: _p,
            normal: Vec3::new(1., 0., 0.),
            front_face: true,
            mat: &self.phase_function,
            // 不改变
            u: 0.,
            v: 0.,
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(_time0, _time1)
    }
}

impl<TA: Hittable, TC: Texture> ConstantMedium<TA, Isotropic<TC>> {
    // 指定为Isotropic的构造函数
    pub fn new(b: TA, d: f64, txt: TC) -> Self {
        Self {
            boundary: b,
            neg_inv_density: (-1. / d),
            phase_function: Isotropic::new(txt),
        }
    }
}

impl<TA: Hittable> ConstantMedium<TA, Isotropic<SolidColor>> {
    // 指定为Isotropic，纹理为 SolidColor的构造函数
    pub fn new_from_color(b: TA, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: (-1. / d),
            phase_function: Isotropic::new_from_color(c),
        }
    }
}
