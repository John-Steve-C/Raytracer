use std::f64::INFINITY;

use crate::{basic_component::vec3::Vec3, hittable::Hittable, utility::random_double};

#[derive(Copy, Clone, Default)]
pub struct Ray {
    pub dir: Vec3,
    pub orig: Vec3,
    pub tm: f64, //光线的出现时间
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn ray_color<T>(r: Ray, background: Vec3, world: &T, depth: i32) -> Vec3
    where
        T: Hittable + 'static,
    {
        // 递归终止条件
        // 超出限制，光无法反射，变成黑色
        if depth <= 0 {
            return Vec3::new(0., 0., 0.);
        }

        let emitted: Vec3;
        // 判断是否碰到物体
        // t_min 修正为 0.01，因为光线并不是在 t=0 处才会击中物体
        if let Some(temp_rec) = world.hit(r, 0.001, INFINITY) {
            emitted = temp_rec.mat.emitted(temp_rec.u, temp_rec.v, temp_rec.p);

            //考虑金属的反射
            if let Some(temp_scatter) = temp_rec.mat.scatter(r, temp_rec) {
                let on_light = Vec3::new(random_double(213., 343.), 554., random_double(227., 332.));
                let mut to_light = on_light - temp_rec.p;
                let distance_squared = to_light.length_squared();
                to_light = Vec3::unit_vector(to_light);

                if Vec3::dot(to_light, temp_rec.normal) < 0. {
                    return emitted;
                }

                let light_area = (343 - 213) * (332 - 227);
                let light_cosine = to_light.y.abs();
                if light_cosine < 0.000001 {
                    return emitted;
                }
                let pdf = distance_squared / (light_cosine * light_area as f64);
                let scattered = Ray::new(temp_rec.p, to_light, r.tm);

                // 如果有，就是二者叠加的颜色
                emitted
                    + temp_scatter.attenuation
                        * temp_rec
                            .mat
                            .scattering_pdf(r, temp_rec, scattered)
                        * Ray::ray_color(scattered, background, world, depth - 1)
                        / pdf
            } else {
                // 金属没有反射，直接发光
                emitted
            }
        } else {
            //没碰到物体，就返回背景的颜色
            // let unit_dir = Vec3::unit_vector(r.dir);
            // let t = 0.5 * (unit_dir.y + 1.);
            // Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t //渐变色
            background
        }
    }

    pub fn new(_ori: Vec3, _d: Vec3, t: f64) -> Self {
        Self {
            dir: _d,
            orig: _ori,
            tm: t,
        }
    }
}
