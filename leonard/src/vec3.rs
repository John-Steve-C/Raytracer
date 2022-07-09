use crate::utility::random_double;
use std::ops::{
    //重载运算符
    Add,
    AddAssign, // + 和 +=
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};

#[derive(Copy, Clone, Default)] // 告诉编译器，这个类型要实现 copy/clone 的 traits
                                // Default 表示有默认构造
pub struct Vec3 {
    //三维向量
    pub x: f64, //坐标
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    //结构体方法
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Vec3 {
    // 结构体关联函数
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        //点乘
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        //叉乘
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(u: Vec3) -> Vec3 {
        //向量单位化
        u / u.length()
    }

    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        Vec3 {
            x: _x,
            y: _y,
            z: _z,
        }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        //随机生成向量
        Vec3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }

    pub fn random_vec_in_unit_sphere() -> Vec3 {
        //在单位球体内生成一个向量
        loop {
            let p = Vec3::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        //随机单位向量
        Vec3::unit_vector(Vec3::random_vec_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        //半球中的随机向量
        let in_unit_sphere = Vec3::random_vec_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0. {
            //和法线在同一个半球
            in_unit_sphere
        } else {
            Vec3::new(0., 0., 0.) - in_unit_sphere
        }
    }
}

// 利用 traits 重载运算符
impl Add<Vec3> for Vec3 {
    // +
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
