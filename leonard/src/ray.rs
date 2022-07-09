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
            //考虑反射，沿球内部随机的 target 点和 p点 的连线发生反射
            // t_min 修正为 0.01，因为光线并不是在 t=0 处才会击中物体
            let target = temp_rec.p + Vec3::random_in_hemisphere(temp_rec.normal);
            Ray::ray_color(
                Ray {
                    dir: (target - temp_rec.p),
                    orig: (temp_rec.p),
                },
                world,
                depth - 1,
            ) * 0.5
        } else {
            //背景的颜色
            let unit_dir = Vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_dir.y + 1.);
            Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}
