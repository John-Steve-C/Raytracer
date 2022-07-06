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
    pub fn hit_sphere(center : Vec3, radius : f64, r : Ray) -> f64{
        let oc = r.orig - center;
        let a = Vec3::dot(r.dir, r.dir);
        let b = 2.0 * Vec3::dot(oc, r.dir);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b *b - 4. * a * c;
        if discriminant < 0. { 
            -1.
        } else {
            (- b - discriminant.sqrt()) / (2. * a) //求解出具体的根
        }
    } 

    pub fn ray_color(r : Ray) -> Vec3{
        let t =  Ray::hit_sphere(Vec3::new(0., 0., -1.), 0.5, r);
        if t > 0. {
            let n = Vec3::unit_vector(r.at(t) -  Vec3::new(0., 0., -1.));
            //求出法向量
            return Vec3::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5;
        }
        let unit_dir = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_dir.y + 1.0);
        return Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t;
    }
}