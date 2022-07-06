use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub dir : Vec3,
    pub orig : Vec3
}

impl Ray {
    pub fn at(&self, t : f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn ray_color(r : Ray) -> Vec3{
        let unit_dir = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
    }
}