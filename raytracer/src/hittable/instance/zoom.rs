use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    optimization::aabb::AABB,
};
pub struct Zoom<T>
// 缩放，改变物体大小
where
    T: Hittable,
{
    pub factor: Vec3, // 在三个方向的缩放系数
    pub now_box: T,
}

impl<T: Hittable> Hittable for Zoom<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray {
            orig: r.orig / self.factor,
            dir: r.dir,
            tm: r.tm,
        };
        if let Some(mut rec) = self.now_box.hit(moved_ray, t_min, t_max) {
            rec.p = rec.p * self.factor;
            rec.set_face_normal(moved_ray, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        if let Some(mut outbox) = self.now_box.bounding_box(_time0, _time1) {
            outbox.minimum = outbox.minimum * self.factor;
            outbox.maximum = outbox.maximum * self.factor;
            Some(outbox)
        } else {
            None
        }
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        self.now_box.pdf_value(o, v) + self.factor.length()
    }

    fn random(&self, o: Vec3) -> Vec3 {
        self.now_box.random(o)
    }
}

impl<T: Hittable> Zoom<T> {
    pub fn new(before: T, fac: Vec3) -> Self {
        Self {
            factor: fac,
            now_box: before,
        }
    }
}
