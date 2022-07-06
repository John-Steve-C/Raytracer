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
    pub fn hit_sphere(center : Vec3, radius : f64, r : Ray) -> bool{
        let oc = r.orig - center;
        let a = Vec3::dot(r.dir, r.dir);
        let b = 2.0 * Vec3::dot(oc, r.dir);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b *b - 4. * a * c;
        discriminant > 0.
    } 

    pub fn ray_color(r : Ray) -> Vec3{
        if Ray::hit_sphere(Vec3::new(0., 0., -1.), 0.5, r) {
            return Vec3::new(1., 0., 0.);
        }
        let unit_dir = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_dir.y + 1.0);
        return Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t;
    }
}