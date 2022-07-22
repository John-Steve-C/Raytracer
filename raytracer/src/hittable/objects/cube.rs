use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{
        objects::aarect::{XYRect, XZRect, YZRect},
        HitRecord, Hittable, HittableList,
    },
    material::Material,
    optimization::aabb::AABB,
};

#[derive(Default)]
pub struct Cube {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Hittable for Cube {
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

impl Cube {
    pub fn new<T: 'static + Copy + Material>(p0: Vec3, p1: Vec3, mat: T) -> Self {
        let mut _sides: HittableList = Default::default();
        _sides.add(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, mat));
        _sides.add(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, mat));
        _sides.add(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, mat));
        _sides.add(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, mat));
        _sides.add(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, mat));
        _sides.add(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, mat));

        Self {
            box_min: p0,
            box_max: p1,
            sides: _sides,
        }
    }
}
