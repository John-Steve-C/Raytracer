use std::f64::{consts::PI, INFINITY};

use crate::{
    basic_component::{onb::ONB, ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    material::Material,
    optimization::aabb::AABB,
    utility::random_to_sphere,
};

#[derive(Clone, Copy)]
pub struct Sphere<T>
where
    T: Material,
{
    pub center: Vec3,
    pub radius: f64,
    pub mat: T, //不保存指针，直接保存结构体
}

impl<T: Material> Hittable for Sphere<T> {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            //判别式
            Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a; //求出方程的根
            if root < t_min || t_max < root {
                //不在范围内，无颜色
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return Option::None;
                }
            }

            let mut rec = HitRecord {
                t: root,
                p: r.at(root),
                normal: Vec3::new(0., 0., 0.),
                front_face: true,
                mat: &self.mat,
                u: 0.,
                v: 0.,
            };
            let outward_normal = (rec.p - self.center) / self.radius; //向外的法向量
            rec.set_face_normal(r, outward_normal);
            rec.get_sphere_uv(outward_normal);

            Option::Some(rec)
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let Some(_rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let cos_theta_max =
                (1. - self.radius * self.radius / ((self.center - o).length_squared())).sqrt();
            let solid_angle = 2. * PI * (1. - cos_theta_max);

            1. / solid_angle
        } else {
            0.
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let dir = self.center - o;
        let distance_squared = dir.length_squared();
        let uvw = ONB::build_from_w(dir);

        uvw.local_from_vec(random_to_sphere(self.radius, distance_squared))
    }
}

impl<T: Material> Sphere<T> {
    pub fn new(_cen: Vec3, r: f64, _mat: T) -> Self {
        Self {
            center: _cen,
            radius: r,
            mat: _mat,
        }
    }
}

//--------------------moving_sphere--------------

#[derive(Clone)]
pub struct MovingSphere<T>
where
    T: Material,
{
    pub radius: f64,
    pub mat: T,        //不保存指针，直接保存结构体
    pub center0: Vec3, //在始末时间的球心位置
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
}

impl<T: Material> Hittable for MovingSphere<T> {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.get_center(r.tm);
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            //判别式
            Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a; //求出方程的根
            if root < t_min || t_max < root {
                //不在范围内，无颜色
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return Option::None;
                }
            }

            let mut rec = HitRecord {
                t: root,
                p: r.at(root),
                normal: Vec3::new(0., 0., 0.),
                front_face: true,
                mat: &self.mat,
                u: 0.,
                v: 0.,
            };
            let outward_normal = (rec.p - self.get_center(r.tm)) / self.radius; //向外的法向量
            rec.set_face_normal(r, outward_normal);
            rec.get_sphere_uv(outward_normal);

            Option::Some(rec)
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // 取始末位置的两个球，找到包裹的空间
        let box0 = AABB {
            minimum: self.get_center(_time0) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.get_center(_time0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            minimum: self.get_center(_time1) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.get_center(_time1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(AABB::surrounding_box(box0, box1))
    }
}

impl<T: Material> MovingSphere<T> {
    pub fn get_center(&self, time: f64) -> Vec3 {
        // 球心的位置随时间线性变化
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }

    pub fn new(r: f64, _c0: Vec3, _c1: Vec3, _t0: f64, _t1: f64, _mat: T) -> Self {
        Self {
            radius: r,
            mat: _mat,
            center0: _c0,
            center1: _c1,
            time0: _t0,
            time1: _t1,
        }
    }
}
