use std::f64::INFINITY;

use super::{HitRecord, Hittable};
use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    material::Material,
    optimization::aabb::AABB,
    utility::random_double,
};

pub struct XYRect<T>
where
    T: Material,
{
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T: Material> Hittable for XYRect<T> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            maximum: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // 通过 z 平面确定出碰撞方程的 _t
        let _t = (self.k - r.orig.z) / r.dir.z;
        if _t < t_min || _t > t_max {
            return None;
        }

        let x = r.orig.x + _t * r.dir.x;
        let y = r.orig.y + _t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0., 0., 1.);
        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            t: _t,
            mat: &self.mp,
            p: r.at(_t),
            //临时变量，后面由 set_face_normal 决定
            front_face: true,
            normal: Vec3::new(0., 0., 0.),
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

impl<T: Material> XYRect<T> {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, _mp: T) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
            mp: _mp,
        }
    }
}

pub struct XZRect<T>
where
    T: Material,
{
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T: Material> Hittable for XZRect<T> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            maximum: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let _t = (self.k - r.orig.y) / r.dir.y;
        if _t < t_min || _t > t_max {
            return None;
        }

        let x = r.orig.x + _t * r.dir.x;
        let z = r.orig.z + _t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(0., 1., 0.);
        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t: _t,
            mat: &self.mp,
            p: r.at(_t),
            front_face: true,
            normal: Vec3::new(0., 0., 0.),
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        // ray 的 t 是否为 0？
        if let Some(rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (Vec3::dot(v, rec.normal) / v.length()).abs();

            distance_squared / (cosine * area)
        } else {
            0.
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_double(self.x0, self.x1),
            self.k,
            random_double(self.z0, self.z1),
        );
        random_point - o
    }
}

impl<T: Material> XZRect<T> {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, _mp: T) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: _mp,
        }
    }
}
pub struct YZRect<T>
where
    T: Material,
{
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: T,
}

impl<T: Material> Hittable for YZRect<T> {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            maximum: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let _t = (self.k - r.orig.x) / r.dir.x;
        if _t < t_min || _t > t_max {
            return None;
        }

        let y = r.orig.y + _t * r.dir.y;
        let z = r.orig.z + _t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(1., 0., 0.);
        let mut rec = HitRecord {
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t: _t,
            mat: &self.mp,
            p: r.at(_t),
            front_face: true,
            normal: Vec3::new(0., 0., 0.),
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

impl<T: Material> YZRect<T> {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, _mp: T) -> Self {
        Self {
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: _mp,
        }
    }
}
