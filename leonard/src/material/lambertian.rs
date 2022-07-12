use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    texture::Texture,
};

#[derive(Clone)]
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
            scattered: Ray {
                dir: scatter_dir,
                orig: rec.p,
                tm: _r_in.tm,
            },
            attenuation: self.albedo.get_color_value(rec.u, rec.v, rec.p),
        })
    }
}
