use std::f64::INFINITY;

use crate::{basic_component::vec3::Vec3, hittable::Hittable};

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
    pub fn ray_color<T>(r: Ray, world: &T, depth: i32) -> Vec3
    where
        T: Hittable + 'static,
    {
        // 递归终止条件
        // 超出限制，光无法反射，变成黑色
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }

        if let Some(temp_rec) = world.hit(r, 0.001, INFINITY) {
            //考虑金属的反射
            // t_min 修正为 0.01，因为光线并不是在 t=0 处才会击中物体
            if let Some(temp_scatter) = temp_rec.mat.scatter(r, temp_rec) {
                Ray::ray_color(temp_scatter.scattered, world, depth - 1) * temp_scatter.attenuation
            } else {
                Vec3::new(0., 0., 0.)
            }
        } else {
            //背景的颜色
            let unit_dir = Vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_dir.y + 1.);
            Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}
