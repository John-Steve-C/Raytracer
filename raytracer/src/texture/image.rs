use super::Texture;
use crate::{basic_component::vec3::Vec3, utility::clamp};
use image::RgbImage;
use std::sync::Arc;

pub struct ImageTexture {
    //贴图
    pub image: Arc<RgbImage>,
    // 每次传图片文件，会爆空间
    // 所以要用指针，节约时间
}

impl Texture for ImageTexture {
    fn get_color_value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        // 防止空文件
        // if self.data.is_empty() {return Vec3::new(0., 1., 1.);}
        // 修改为图像坐标
        let uu = clamp(u, 0., 1.);
        let vv = 1. - clamp(v, 0., 1.);

        let mut i = (uu * self.image.width() as f64) as u32;
        let mut j = (vv * self.image.height() as f64) as u32;
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

impl ImageTexture {
    pub fn new_from_file(file_name: &str) -> Self {
        let tmp_file;
        match image::open(file_name) {
            //文件读入
            Ok(ret) => tmp_file = ret,
            Err(_) => panic!("file doesn't exist!"),
        }

        ImageTexture {
            image: Arc::new(tmp_file.into_rgb8()),
        }
    }
}
