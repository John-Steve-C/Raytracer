use super::{HitRecord, Hittable};
use crate::{basic_component::ray::Ray, optimization::aabb::AABB};

pub struct Flipface<T>
where
    T: Hittable,
{
    pub ptr: T,
}

impl<T: Hittable> Hittable for Flipface<T> {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.ptr.bounding_box(time0, time1)
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }
}

impl<T: Hittable> Flipface<T> {
    pub fn new(p: T) -> Self {
        Self { ptr: p }
    }
}
