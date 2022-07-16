use super::Texture;
use crate::basic_component::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct CheckerTexture<T>
// 实现棋盘的颜色
where
    T: Texture,
{
    pub odd: T, //间隔的格子颜色不同
    pub even: T,
}

impl<T: Texture> Texture for CheckerTexture<T> {
    fn get_color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.get_color_value(u, v, p)
        } else {
            self.even.get_color_value(u, v, p)
        }
    }
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(_odd: T, _even: T) -> Self {
        Self {
            odd: _odd,
            even: _even,
        }
    }
}
