use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable},
    optimization::aabb::AABB,
};

pub struct Translate<T>
where
    T: Hittable,
{
    pub offset: Vec3,
    pub after_box: T,
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            tm: r.tm,
        };
        if let Some(mut rec) = self.after_box.hit(moved_ray, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(moved_ray, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        if let Some(mut outbox) = self.after_box.bounding_box(_time0, _time1) {
            outbox.minimum += self.offset;
            outbox.maximum += self.offset;
            Some(outbox)
        } else {
            None
        }
    }
}

impl<T: Hittable> Translate<T> {
    pub fn new(_after: T, _off: Vec3) -> Self {
        Self {
            offset: _off,
            after_box: _after,
        }
    }
}
