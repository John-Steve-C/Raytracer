use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
    //材料对光线的反射情况
}

pub struct ScatterRecord {
    //保存反射的结果
    pub attenuation: Vec3, //衰减系数
    pub scattered: Ray,    //反射光
}

pub struct Lambertian {
    pub albedo: Vec3, //反照率
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        //过小时，修正
        Some(ScatterRecord {
            scattered: Ray {
                dir: scatter_dir,
                orig: rec.p,
            },
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,   //反照率
    pub fuzz : f64      //模糊度，让反射方向做出细微改变
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let _scattered = Ray {
            dir: reflected + Vec3::random_vec_in_unit_sphere() * self.fuzz, //模糊化反射
            orig: rec.p,
        };
        let _attenuation = self.albedo;

        if Vec3::dot(_scattered.dir, rec.normal) > 0. {
            Some(ScatterRecord {
                scattered: _scattered,
                attenuation: _attenuation,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric{ //电介质
    pub ir : f64 //折射系数
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let mut refraction_ratio = self.ir;
        if rec.front_face {refraction_ratio = 1. / self.ir;}

        let unit_dir = Vec3::unit_vector(r_in.dir);

        let mut cos_theta = Vec3::dot(Vec3::new(0., 0., 0.) - unit_dir, rec.normal);
        if cos_theta > 1. {cos_theta = 1.;}
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let dir : Vec3;
        if cannot_refract {
            dir = Vec3::reflect(unit_dir, rec.normal);
        } else {
            dir = Vec3::refract(unit_dir, rec.normal, refraction_ratio)
        }

        Some(ScatterRecord{
            scattered : Ray{
                dir: dir,
                orig: rec.p
            },
            attenuation: Vec3::new(1., 1., 1.)
        })
    }
}