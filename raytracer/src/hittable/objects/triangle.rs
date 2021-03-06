use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    material::{metal::Metal, Material},
    optimization::aabb::AABB,
    utility::random_double,
};

#[derive(Clone, Copy)]
pub struct Triangle<T>
where
    T: Material,
{
    pub vers: [Vec3; 3], // 三个顶点
    pub mat: T,
    pub normal: Vec3,
    pub center: Vec3,
    pub area: f64,
    // 用来判断点是否在三角形内部的辅助变量
    v: Vec3,
    w: Vec3,
}

impl<T: Material> Triangle<T> {
    pub fn new(point: [Vec3; 3], _mat: T) -> Self {
        let _i = point[1] - point[0];
        let _j = point[2] - point[0];
        // 表示三角形所在的平面，用来判断是否相交
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
            v: _v,
            w: _w,
        }
    }
}

impl<T: Material> Hittable for Triangle<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // 先判断光线是否与三角形共面
        let _t = Vec3::dot(self.center - r.orig, self.normal) / Vec3::dot(r.dir, self.normal);
        if _t.is_nan() || _t < t_min || _t > t_max {
            // 线面平行(除数为0)，不符
            return None;
        }

        let p = (r.orig + r.dir * _t) - self.vers[0];
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
                        p: r.at(_t),
                        t: _t,
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

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let Some(rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let area = self.area;
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (Vec3::dot(v, rec.normal) / v.length()).abs();

            distance_squared / (cosine * area)
        } else {
            0.
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let i = self.vers[1] - self.vers[0];
        let j = self.vers[2] - self.vers[0];
        let mut k1 = random_double(0., 1.);
        let mut k2 = random_double(0., 1.);
        if k1 + k2 > 1. {
            k1 = 1. - k1;
            k2 = 1. - k2;
        }

        (i * k1 + j * k2) - o
    }
}

#[derive(Clone, Copy)]
pub struct OBJTriangle<T>
where
    T: Material,
{
    pub vers: [Vec3; 3], // 三个顶点
    pub mat: T,
    pub normal: Vec3,
    pub center: Vec3,
    pub area: f64,
    // 用来判断点是否在三角形内部的辅助变量
    v: Vec3,
    w: Vec3,
    pub texs: [(f64, f64); 3],

    // 用于特判
    pub aluminum: Metal,
    pub black: Metal,
    pub yellow: Metal,
    pub is_al: bool,
    pub is_black: bool,
    pub is_yellow: bool,
}

impl<T: Material> OBJTriangle<T> {
    pub fn new(
        point: [Vec3; 3],
        _texs: [(f64, f64); 3],
        _mat: T,
        _is_al: bool,
        _is_black: bool,
        _is_yellow: bool,
    ) -> Self {
        let _i = point[1] - point[0];
        let _j = point[2] - point[0];
        // 表示三角形所在的平面，用来判断是否相交
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
            v: _v,
            w: _w,
            texs: _texs,
            aluminum: Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.),
            black: Metal::new(Vec3::new(0.1, 0.1, 0.1), 0.),
            yellow: Metal::new(Vec3::new(0.925, 0.788, 0.251), 0.),
            is_al: _is_al,
            is_black: _is_black,
            is_yellow: _is_yellow,
        }
    }

    pub fn get_dis(&self, hit_point: Vec3) -> [f64; 3] {
        // 利用碰撞点到中心的距离进行加权
        let mut c = [0.; 3];
        c[0] = (hit_point - self.vers[0]).length();
        c[1] = (hit_point - self.vers[1]).length();
        c[2] = (hit_point - self.vers[2]).length();
        let tot = c[0] + c[1] + c[2];

        [c[0] / tot, c[1] / tot, c[2] / tot]
    }

    pub fn get_cord(&self, hit_point: Vec3) -> [f64; 3] {
        // 另一种求加权值的方法（重心坐标系）
        let mut n: [Vec3; 3] = Default::default();
        let area_vec = Vec3::cross(self.vers[1] - self.vers[0], self.vers[2] - self.vers[0]);
        n[0] = Vec3::cross(self.vers[2] - self.vers[1], hit_point - self.vers[1]);
        n[1] = Vec3::cross(self.vers[0] - self.vers[2], hit_point - self.vers[2]);
        n[2] = Vec3::cross(self.vers[1] - self.vers[0], hit_point - self.vers[0]);

        let mut c: [f64; 3] = Default::default();
        c[0] = Vec3::dot(n[0], area_vec) / area_vec.length().powi(2);
        c[1] = Vec3::dot(n[1], area_vec) / area_vec.length().powi(2);
        c[2] = Vec3::dot(n[2], area_vec) / area_vec.length().powi(2);
        c
    }
}

impl<T: Material> Hittable for OBJTriangle<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // 先判断光线是否与三角形共面
        let _t = Vec3::dot(self.center - r.orig, self.normal) / Vec3::dot(r.dir, self.normal);
        if _t.is_nan() || _t < t_min || _t > t_max {
            // 线面平行(除数为0)，不符
            return None;
        }

        let p = (r.orig + r.dir * _t) - self.vers[0];
        // 仿射坐标系计算
        // 参考 https://blog.csdn.net/gyb641393267/article/details/48860189
        let gamma = Vec3::dot(p, self.v);
        if gamma > 0. && gamma < 1. {
            let beta = Vec3::dot(p, self.w);
            if beta > 0. && beta < 1. {
                let alpha = 1. - gamma - beta;
                if alpha > 0. && alpha < 1. {
                    // 确定出 该点 在三角形内
                    let c = self.get_cord(r.at(_t));
                    let _u = self.texs[0].0 * c[0] + self.texs[1].0 * c[1] + self.texs[2].0 * c[2];
                    let _v = self.texs[0].1 * c[0] + self.texs[1].1 * c[1] + self.texs[2].1 * c[2];
                    // let norm = self.normals[0] * c[0] + self.normals[1] * c[1] + self.normals[2] * c[2];
                    // println!("{}, {}", _u, _v);

                    let mut rec = HitRecord {
                        p: r.at(_t),
                        t: _t,
                        mat: &self.mat,
                        u: _u,
                        v: _v,

                        front_face: true,
                        normal: Vec3::new(0., 0., 0.),
                    };
                    rec.set_face_normal(r, self.normal);

                    // println!("{}, {}", self.is_black, self.is_al);

                    // 为了特化，把材质修改为铝，实现金属材质的反射效果
                    if self.is_al {
                        rec.mat = &self.aluminum;
                    }
                    if self.is_yellow {
                        rec.mat = &self.yellow;
                    }
                    if self.is_black {
                        rec.mat = &self.black;
                    }

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

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let Some(rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let area = self.area;
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (Vec3::dot(v, rec.normal) / v.length()).abs();

            distance_squared / (cosine * area)
        } else {
            0.
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let i = self.vers[1] - self.vers[0];
        let j = self.vers[2] - self.vers[0];
        let mut k1 = random_double(0., 1.);
        let mut k2 = random_double(0., 1.);
        if k1 + k2 > 1. {
            k1 = 1. - k1;
            k2 = 1. - k2;
        }

        (i * k1 + j * k2) - o
    }
}
