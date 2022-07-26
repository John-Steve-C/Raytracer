use super::vec3::Vec3;
use std::ops::{Index, IndexMut};

// #[derive(Default, Clone, Copy)]
pub struct ONB {
    // 正交基
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }

    pub fn local_from_vec(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    #[allow(clippy::many_single_char_names)]
    pub fn build_from_w(n: Vec3) -> Self {
        let w = Vec3::unit_vector(n);
        let a;
        if w.x.abs() > 0.9 {
            a = Vec3::new(0., 1., 0.);
        } else {
            a = Vec3::new(1., 0., 0.);
        }
        let v = Vec3::unit_vector(Vec3::cross(w, a));
        let u = Vec3::cross(w, v);

        Self { axis: [u, v, w] }
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}

impl IndexMut<usize> for ONB {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.axis[index]
    }
}
