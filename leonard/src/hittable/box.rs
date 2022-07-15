use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{
        aarect::{XYRect, XZRect, YZRect},
        HitRecord, Hittable, HittableList,
    },
    material::Material,
    optimization::aabb::AABB,
};

#[derive(Default)]
pub struct Box {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Hittable for Box {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: self.box_min,
            maximum: self.box_max,
        })
    }

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}

impl Box {
    pub fn new<T: 'static + Copy + Material>(p0: Vec3, p1: Vec3, mat: T) -> Self {
        let mut _sides: HittableList = Default::default();
        _sides.add(XYRect {
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p1.z,
            mp: mat,
        });
        _sides.add(XYRect {
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p0.z,
            mp: mat,
        });
        _sides.add(XZRect {
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p1.y,
            mp: mat,
        });
        _sides.add(XZRect {
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p0.y,
            mp: mat,
        });
        _sides.add(YZRect {
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p1.x,
            mp: mat,
        });
        _sides.add(YZRect {
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p0.x,
            mp: mat,
        });

        Self {
            box_min: p0,
            box_max: p1,
            sides: _sides,
        }
    }
}
