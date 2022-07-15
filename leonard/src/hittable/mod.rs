pub mod aarect;
pub mod r#box;
pub mod sphere;
pub mod instance;

use std::f64::consts::PI;

use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    material::Material,
    optimization::aabb::AABB,
};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,               //碰撞点
    pub normal: Vec3,          //法向量
    pub t: f64,                //对应光线的 at(t)
    pub front_face: bool,      //方向是否为外侧
    pub mat: &'a dyn Material, //材料，变量类型是对包含 Material 结构体的引用
    pub u: f64,                // 用来表示纹理
    pub v: f64,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        //求出normal和front_face
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = Vec3::new(0., 0., 0.) - outward_normal;
        }
    }

    pub fn get_sphere_uv(&mut self, p: Vec3) {
        // p 是球上的一点
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        //相当于 arctan(-p.z / p.x)
        self.u = phi / (2. * PI);
        self.v = theta / PI;
    }
}
pub trait Hittable {
    //特性，用于实现继承
    //代替c++中包裹record的类
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    //判断光线在 [t_min, t_max] 内是否碰到物体
    //优化，用 Option 是否为 None 来判断碰撞与否，同时包括返回值
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB>;
    // AABB 优化，判断光线是否撞到 大的 box
}

//------------------------------------

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>, //智能指针 + 特性
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T>(&mut self, obj: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec: Option<HitRecord> = Option::None;
        let mut closest_so_far = t_max;

        //找到离光线最近的图形，这样才能保证画出正确的图
        for i in &self.objects {
            if let Some(temp_rec) = i.hit(r, t_min, closest_so_far) {
                //此处的 temp_rec 相当于 something，只是表示不为 None
                closest_so_far = temp_rec.t;
                hit_rec = Some(temp_rec);
            }
        }

        hit_rec
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = AABB {
            minimum: Vec3::new(0., 0., 0.),
            maximum: Vec3::new(0., 0., 0.),
        };
        let mut is_first_box = true;

        for i in &self.objects {
            if let Some(temp_box) = i.bounding_box(_time0, _time1) {
                if is_first_box {
                    output_box = temp_box;
                } else {
                    output_box = AABB::surrounding_box(output_box, temp_box);
                }
            } else {
                return None;
            }
            is_first_box = false;
        }

        Some(output_box)
    }
}
