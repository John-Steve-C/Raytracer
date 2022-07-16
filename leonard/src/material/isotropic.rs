use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    texture::{solid::SolidColor, Texture},
};

#[derive(Default, Clone)]
pub struct Isotropic<T>
//各向同性
where
    T: Texture,
{
    pub albedo: T,
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, Vec3::random_vec_in_unit_sphere(), r_in.tm),
            attenuation: self.albedo.get_color_value(rec.u, rec.v, rec.p),
        })
    }
}

impl<T: Texture> Isotropic<T> {
    // 构造函数
    pub fn new(c: T) -> Self {
        Self { albedo: c }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_from_color(c: Vec3) -> Self {
        Self {
            albedo: SolidColor { color_value: c },
        }
    }
}
