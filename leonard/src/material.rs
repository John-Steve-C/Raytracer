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
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
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
    pub albedo: Vec3, //反照率
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let _scattered = Ray {
            dir: reflected,
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
