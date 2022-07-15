use std::f64::INFINITY;

use crate::{
    hittable::{Hittable, HitRecord},
    basic_component::{vec3::Vec3, ray::Ray},
    optimization::aabb::AABB,
    utility::{degree_to_radian, min_f64, max_f64},
};

pub struct RotateY<T>   //绕y轴旋转
where
    T : Hittable,
{
    pub sin_theta : f64,
    pub cos_theta : f64,
    pub hasbox : bool,
    pub bbox : AABB,
    pub after_box : T,  //旋转后的结果
}

impl <T : Hittable> Hittable for RotateY<T> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        if self.hasbox {
            Some(self.bbox)
        } else {
            None
        }
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut _orig = r.orig;
        let mut _dir = r.dir;

        _orig[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        _orig[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        _dir[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        _dir[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_ray = Ray{dir : _dir, orig : _orig, tm : r.tm};
        if let Some(mut rec) = self.after_box.hit(rotated_ray, t_min, t_max) {
            let mut _p = rec.p;
            let mut _normal = rec.normal;

            _p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            _p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            _normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            _normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = _p;
            rec.set_face_normal(rotated_ray, _normal);

            Some(rec)
        } else {
            None
        }
    }
}

impl <T : Hittable> RotateY<T>{
    pub fn new(p : T, angle : f64) -> Self{
        let radians = degree_to_radian(angle); //旋转角
        let _sin = radians.sin();
        let _cos = radians.cos();

        let flag;
        let mut tp_box : AABB = Default::default();
        if let Some(_box) = p.bounding_box(0., 1.) {
            flag = true;
            tp_box = _box;
        } else {
            flag = false;
        }

        let mut min_v = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max_v = Vec3::new(INFINITY, INFINITY, INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = tp_box.maximum.x * i as f64 + tp_box.minimum.x * (1 - i) as f64;
                    let y = tp_box.maximum.y * j as f64 + tp_box.minimum.y * (1 - j) as f64;
                    let z = tp_box.maximum.z * k as f64 + tp_box.minimum.z * (1 - k) as f64;
                    // 旋转公式
                    let newx = _cos * x + _sin * z;
                    let newz = -_sin * x + _cos * z;

                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min_v[c] = min_f64(min_v[c], tester[c]);
                        max_v[c] = max_f64(max_v[c], tester[c]);
                    }
                }
            }
        }

        Self { 
            sin_theta: _sin, 
            cos_theta: _cos, 
            hasbox: flag, 
            bbox: AABB { minimum: min_v, maximum: max_v }, 
            after_box: p, 
        }
    }
}