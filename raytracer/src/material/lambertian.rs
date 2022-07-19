use std::f64::consts::PI;

use crate::{
    basic_component::{onb::ONB, ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    optimization::pdf::CosinePDF,
    texture::{solid::SolidColor, Texture},
};

#[derive(Clone, Copy)]
pub struct Lambertian<T>
where
    T: Texture,
{
    //理想散射
    pub albedo: T, //反照率
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        // let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        //防止 scatter_dir 相加后恰好为 0 向量
        // if scatter_dir.near_zero() {
        //     scatter_dir = rec.normal;
        // }
        // 过小时，修正
        // 加上 pdf，选择 importance sampling

        let uvw = ONB::build_from_w(rec.normal);
        let dir = uvw.local_from_vec(Vec3::random_cosine_direction());
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, Vec3::unit_vector(dir), _r_in.tm),
            attenuation: self.albedo.get_color_value(rec.u, rec.v, rec.p),
            cos_pdf: CosinePDF::new(rec.normal),
            pdf_type: 1,
            is_specular: false,
        })
    }

    fn scattering_pdf(&self, _r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.dir));
        if cosine < 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new(c: T) -> Self {
        Self { albedo: c }
    }
}

impl Lambertian<SolidColor> {
    // 指定特定类型的构造函数
    pub fn new_from_color(c: Vec3) -> Self {
        Self {
            albedo: SolidColor { color_value: c },
        }
    }
}
