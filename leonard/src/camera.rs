use crate::{ray::Ray, utility, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,            //原点
    pub lower_left_corner: Vec3, //左下角
    pub horizontal: Vec3,        //水平向量
    pub vertical: Vec3,          //竖直
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64, //镜头半径
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        //充当默认构造函数
        // aspect_ratio 长宽比
        // vfov : vertical field of view in degrees 竖直观测角
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
            origin: (_origin),
            lower_left_corner: (_lower_left_corner),
            horizontal: (_horizontal),
            vertical: (_vertical),
            u: (_u),
            v: (_v),
            w: (_w),
            lens_radius: (_lens_radius),
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
        }
    }
}
