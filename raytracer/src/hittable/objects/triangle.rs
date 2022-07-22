use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    material::Material,
    optimization::aabb::AABB,
    utility::random_double,
};

pub struct Triangle<T>
where
    T: Material,
{
    pub vers: [Vec3; 3], // 三个顶点
    pub mat: T,
    pub normal: Vec3,
    pub center: Vec3,
    pub area: f64,
    // 表示三角形所在的平面，用来判断是否相交
    i: Vec3,
    j: Vec3,
    // 用来判断点是否在三角形内部的辅助变量
    v: Vec3,
    w: Vec3,
}

impl<T: Material> Triangle<T> {
    pub fn new(point: [Vec3; 3], _mat: T) -> Self {
        let _i = point[1] - point[0];
        let _j = point[2] - point[0];
        let _center = (point[0] + point[1] + point[2]) / 3.;
        let mut _normal = Vec3::cross(_i, _j);
        let _area = _normal.length() / 2.;
        _normal = Vec3::unit_vector(_normal);

        let mut _v = Vec3::cross(_normal, _i);
        _v /= Vec3::dot(_j, _v);
        let mut _w = Vec3::cross(_normal, _j);
        _w /= Vec3::dot(_i, _w);

        Self {
            vers: point,
            mat: _mat,
            normal: _normal,
            center: _center,
            area: _area,
            i: _i,
            j: _j,
            v: _v,
            w: _w,
        }
    }
}

impl<T: Material> Hittable for Triangle<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // 先判断光线是否与三角形共面
        let t = Vec3::dot(self.center - r.orig, self.normal) / Vec3::dot(r.dir, self.normal);
        if t.is_nan() || t < t_min || t > t_max {
            // 线面平行(除数为0)，不符
            return None;
        }

        let p = (r.orig + r.dir * t) - self.vers[0];
        // 仿射坐标系计算
        // 参考 https://blog.csdn.net/gyb641393267/article/details/48860189
        let gamma = Vec3::dot(p, self.v);
        if gamma > 0. && gamma < 1. {
            let beta = Vec3::dot(p, self.w);
            if beta > 0. && beta < 1. {
                let alpha = 1. - gamma - beta;
                if alpha > 0. && alpha < 1. {
                    // 确定出 该点 在三角形内
                    let mut rec = HitRecord {
                        p: r.at(t),
                        t: t,
                        mat: &self.mat,
                        u: alpha,
                        v: beta,

                        front_face: true,
                        normal: Vec3::new(0., 0., 0.),
                    };
                    rec.set_face_normal(r, self.normal);
                    return Some(rec);
                }
            }
        }

        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);
        for t in &self.vers {
            if t.x > max.x {
                max.x = t.x;
            }
            if t.y > max.y {
                max.y = t.y;
            }
            if t.z > max.z {
                max.z = t.z;
            }

            if t.x < min.x {
                min.x = t.x;
            }
            if t.y < min.y {
                min.y = t.y;
            }
            if t.z < min.z {
                min.z = t.z;
            }
        }

        Some(AABB {
            minimum: min,
            maximum: max,
        })
    }

    // fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f64 {

    // }

    // fn random(&self, o: Vec3) -> Vec3 {
    //     let mut k1 = random_double(0., 1.);
    //     let mut k2 = random_double(0., 1.);
    //     if k1 + k2 > 1. {
    //         k1 = 1. - k1;
    //         k2 = 1. - k2;
    //     }

    //     (self.i * k1 + self.j * k2) - o
    // }
}
