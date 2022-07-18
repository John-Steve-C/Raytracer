use std::f64::consts::PI;

use crate::{
    basic_component::{onb::ONB, vec3::Vec3},
    hittable::Hittable,
};

pub trait PDF {
    fn value(&self, _d: Vec3) -> f64 {
        0.
    }

    fn generate(&self) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}

//------------------------------------------------------------------

pub struct CosinePDF {
    pub uvw: ONB,
}

impl PDF for CosinePDF {
    fn generate(&self) -> Vec3 {
        self.uvw.local_from_vec(Vec3::random_cosine_direction())
    }

    fn value(&self, d: Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(d), self.uvw.w());
        if cosine <= 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

//------------------------------------------------------------------------

pub struct HittablePDF<'a, T>
where
    T: Hittable,
{
    o: Vec3,
    ptr: &'a T,
    // 引用，需要考虑生命周期
}

impl<'a, T: Hittable> PDF for HittablePDF<'a, T> {
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }

    fn value(&self, _d: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, _d)
    }
}

impl<'a, T: Hittable> HittablePDF<'a, T> {
    pub fn new(p: &'a T, orig: Vec3) -> Self {
        Self { o: orig, ptr: p }
    }
}
