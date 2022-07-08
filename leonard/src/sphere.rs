use crate::{
    vec3::Vec3,
    ray::Ray,
    hittable::Hittable,
    hittable::HitRecord
};

pub struct Sphere{
    pub center : Vec3,
    pub radius : f64
}

impl Hittable for Sphere {
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord>{
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            Option::None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a; //求出方程的根
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return Option::None
                }
            }

            let mut rec = HitRecord{
                t : root,
                p : r.at(root),
                normal : Vec3::new(0., 0., 0.),
                front_face : true
            };
            rec.normal = (rec.p - self.center) / self.radius;
            let outward_normal = rec.p - self.center;
            rec.set_face_normal(r, outward_normal);

            Option::Some(rec)
        }
    }
}