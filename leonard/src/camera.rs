use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16. / 9.;
        let view_height = 2.;
        let view_width = aspect_ratio * view_height;
        let focal_length = 1.;

        let origin = Vec3::new(0., 0., 0.);
        let horizontal = Vec3::new(view_width, 0., 0.);
        let vertical = Vec3::new(0., view_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);
        Camera {
            origin: (origin),
            lower_left_corner: (lower_left_corner),
            horizontal: (horizontal),
            vertical: (vertical),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            dir: (self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
            orig: (self.origin),
        }
    }
}
