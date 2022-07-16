use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
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
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        //防止 scatter_dir 相加后恰好为 0 向量
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        //过小时，修正
        Some(ScatterRecord {
            scattered: Ray::new(rec.p, scatter_dir, _r_in.tm),
            attenuation: self.albedo.get_color_value(rec.u, rec.v, rec.p),
        })
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
