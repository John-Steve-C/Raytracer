use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray, vec3::Vec3};

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
}
