use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    material::Material,
    optimization::aabb::AABB,
};

#[derive(Clone)]
pub struct Sphere<T>
where
    T: Material,
{
    pub center: Vec3,
    pub radius: f64,
    pub mat: T, //不保存指针，直接保存结构体
}

impl<T: Material> Hittable for Sphere<T> {
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
            };
            let outward_normal = (rec.p - self.center) / self.radius; //向外的法向量
            rec.set_face_normal(r, outward_normal);

            Option::Some(rec)
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
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
            };
            let outward_normal = (rec.p - self.get_center(r.tm)) / self.radius; //向外的法向量
            rec.set_face_normal(r, outward_normal);

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
}
