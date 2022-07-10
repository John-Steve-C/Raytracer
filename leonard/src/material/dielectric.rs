use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    utility::random_double,
};

#[derive(Clone, Copy)]
pub struct Dielectric {
    //电介质
    pub ir: f64, //折射系数
}

impl Dielectric {
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        //利用 Schlick's approximation 进行估计
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let mut refraction_ratio = self.ir;
        if rec.front_face {
            refraction_ratio = 1. / self.ir;
        }

        let unit_dir = Vec3::unit_vector(r_in.dir);

        let mut cos_theta = Vec3::dot(Vec3::new(0., 0., 0.) - unit_dir, rec.normal);
        if cos_theta > 1. {
            cos_theta = 1.;
        }
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let _dir: Vec3;
        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double(0., 1.)
        {
            _dir = Vec3::reflect(unit_dir, rec.normal);
        } else {
            _dir = Vec3::refract(unit_dir, rec.normal, refraction_ratio)
        }

        Some(ScatterRecord {
            scattered: Ray {
                dir: _dir,
                orig: rec.p,
            },
            attenuation: Vec3::new(1., 1., 1.),
        })
    }
}
