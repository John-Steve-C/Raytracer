use super::{HitRecord, Hittable};
use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    material::Material,
    optimization::aabb::AABB,
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
