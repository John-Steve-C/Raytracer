use crate::basic_component::vec3::Vec3;

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
    pub fn new(_ori: Vec3, _d: Vec3, t: f64) -> Self {
        Self {
            dir: _d,
            orig: _ori,
            tm: t,
        }
    }
}
