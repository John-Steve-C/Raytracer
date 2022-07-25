use super::Texture;
use crate::basic_component::vec3::Vec3;
use image::RgbImage;

pub struct OBJTexture {
    //贴图
    pub image: RgbImage,
    pub u: f64,
    pub v: f64,
}

impl Texture for OBJTexture {
    fn get_color_value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        let mut i = (self.u * self.image.width() as f64) as u32;
        let mut j = (self.v * self.image.height() as f64) as u32;
        // 每次都取固定的点
        if i >= self.image.width() {
            i = self.image.width() - 1;
        }
        if j >= self.image.height() {
            j = self.image.height() - 1;
        }

        let color_scale = 1. / 255.;
        Vec3::new(
            self.image.get_pixel(i, j).0[0] as f64 * color_scale,
            self.image.get_pixel(i, j).0[1] as f64 * color_scale,
            self.image.get_pixel(i, j).0[2] as f64 * color_scale,
        )
    }
}

impl OBJTexture {
    pub fn new_from_file(file_name: &str, _u: f64, _v: f64) -> Self {
        let tmp_file;
        match image::open(file_name) {
            //文件读入
            Ok(ret) => tmp_file = ret,
            Err(_) => panic!("file doesn't exist!"),
        }

        OBJTexture {
            image: tmp_file.into_rgb8(),
            u: _u,
            v: _v,
        }
    }
}
