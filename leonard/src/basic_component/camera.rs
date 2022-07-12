use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    utility::{self, random_double},
};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,            //原点
    pub lower_left_corner: Vec3, //左下角
    pub horizontal: Vec3,        //水平向量
    pub vertical: Vec3,          //竖直
    pub u: Vec3,                 //用来描述相机方向的正交基
    pub v: Vec3,
    pub w: Vec3,          //也是相机的朝向
    pub lens_radius: f64, //镜头半径
    pub time0: f64,       //拍摄的始末时间
    pub time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3, //相机放置的位置
        lookat: Vec3,   //相机朝向的点
        vup: Vec3,      //与光线正交的“向上”向量，用来计算镜头的旋转角度
        vfov: f64,      //vertical field of view in degrees 观察的广角（越小，看到的东西越少、大）
        aspect_ratio: f64,
        aperture: f64, //光圈
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Camera {
        //充当默认构造函数
        // aspect_ratio 长宽比
        let theta = utility::degree_to_radian(vfov);
        let h = (theta / 2.).tan();
        let view_height = 2. * h;
        let view_width = aspect_ratio * view_height;

        // let focal_length = 1.; //焦距
        let _w = Vec3::unit_vector(lookfrom - lookat);
        let _u = Vec3::unit_vector(Vec3::cross(vup, _w));
        let _v = Vec3::cross(_w, _u);

        let _origin = lookfrom;
        let _horizontal = _u * view_width * focus_dist;
        let _vertical = _v * view_height * focus_dist;
        let _lower_left_corner = _origin - _horizontal / 2. - _vertical / 2. - _w * focus_dist;

        let _lens_radius = aperture / 2.;

        Camera {
            origin: _origin,
            lower_left_corner: _lower_left_corner,
            horizontal: _horizontal,
            vertical: _vertical,
            u: _u,
            v: _v,
            w: _w,
            lens_radius: _lens_radius,
            time0: _time0,
            time1: _time1,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            dir: (self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset),
            orig: (self.origin + offset),
            tm: random_double(self.time0, self.time1),
            // 光线在拍摄时间内随机生成
        }
    }
}
