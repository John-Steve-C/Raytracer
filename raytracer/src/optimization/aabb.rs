use std::mem::swap;

use crate::basic_component::{ray::Ray, vec3::Vec3};
use crate::utility::{max_f64, min_f64};
// use std::cmp::min;

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1. / r.dir[i];
            let mut t0 = (self.minimum[i] - r.orig[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.orig[i]) * inv_d;

            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            t_min = max_f64(t0, t_min);
            t_max = min_f64(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        //求出包裹了 box0 和 box1 的大箱子
        let small = Vec3::new(
            min_f64(box0.minimum.x, box1.minimum.x),
            min_f64(box0.minimum.y, box1.minimum.y),
            min_f64(box0.minimum.z, box1.minimum.z),
        );
        let big = Vec3::new(
            max_f64(box0.maximum.x, box1.maximum.x),
            max_f64(box0.maximum.y, box1.maximum.y),
            max_f64(box0.maximum.z, box1.maximum.z),
        );

        AABB {
            minimum: small,
            maximum: big,
        }
    }
}
