use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    optimization::pdf::CosinePDF,
};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3, //反照率
    pub fuzz: f64,    //模糊度，让反射方向做出细微改变
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let _scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_vec_in_unit_sphere() * self.fuzz, //模糊化反射
            r_in.tm,
        );
        let _attenuation = self.albedo;

        if Vec3::dot(_scattered.dir, rec.normal) > 0. {
            Some(ScatterRecord {
                scattered: _scattered,
                attenuation: _attenuation,
                cos_pdf: CosinePDF::empty(),
                pdf_type: 0,
                is_specular: true,
            })
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(_alb: Vec3, _fuzz: f64) -> Self {
        Self {
            albedo: _alb,
            fuzz: _fuzz,
        }
    }
}
