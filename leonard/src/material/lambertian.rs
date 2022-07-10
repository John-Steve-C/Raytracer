use crate::{
    basic_component::{
        ray::Ray,
        vec3::Vec3,
    },
    hittable::HitRecord,
    material::{Material, ScatterRecord}
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    //理想散射
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
