use std::f64::consts::PI;

use crate::{
    basic_component::{onb::ONB, vec3::Vec3},
    hittable::Hittable,
    utility::random_double,
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

//-------------------------------------------------------------------

pub struct MixturePDF<'a, T>
// 将两类 pdf 线性混合
where
    T: Hittable,
{
    pub p0: &'a HittablePDF<'a, T>,
    pub p1: CosinePDF,
}

impl<'a, T: Hittable> PDF for MixturePDF<'a, T> {
    fn generate(&self) -> Vec3 {
        // p = 0.5
        if random_double(0., 1.) < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }

    fn value(&self, d: Vec3) -> f64 {
        0.5 * self.p0.value(d) + 0.5 * self.p1.value(d)
    }
}

impl<'a, T: Hittable> MixturePDF<'a, T> {
    pub fn new(_p0: &'a HittablePDF<'a, T>, _p1: CosinePDF) -> Self {
        Self { p0: _p0, p1: _p1 }
    }
}
