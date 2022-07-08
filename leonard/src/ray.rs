use std::f64::INFINITY;

use crate::{hittable::Hittable, vec3::Vec3};

#[derive(Copy, Clone, Default)]
pub struct Ray {
    pub dir: Vec3,
    pub orig: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn ray_color<T>(r: Ray, world: &T) -> Vec3
    where
        T: Hittable + 'static,
    {
        if let Some(temp_rec) = world.hit(r, 0., INFINITY) {
            (temp_rec.normal + Vec3::new(1., 1., 1.)) * 0.5
        } else { //背景的颜色
            let unit_dir = Vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_dir.y + 1.);
            Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}
