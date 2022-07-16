use std::{cmp::Ordering};

use crate::{
    hittable::{Hittable, HittableList},
    optimization::aabb::AABB,
    utility::random_int,
};

pub struct BvhNode {
    // 用来二分查找，作为查找树的结点
    pub r#box: AABB,                     // box 是原有的关键字
    pub left: Option<Box<dyn Hittable>>, //为了实现空指针，必须用 option
    pub right: Option<Box<dyn Hittable>>,
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: crate::basic_component::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.r#box.hit(r, t_min, t_max) {
            return None;
        }

        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        if let Some(hit_left) = self.left.as_ref().unwrap().hit(r, t_min, closest_so_far) {
            closest_so_far = hit_left.t;
            hit_rec = Some(hit_left);
            //顺序不能调换，否则 hit_left 已经被移动，所有权变化
        }
        // 必须先判定 右分支是否为 None
        if self.right.is_some() {
            if let Some(hit_right) = self.right.as_ref().unwrap().hit(r, t_min, closest_so_far) {
                hit_rec = Some(hit_right);
            }
        }

        hit_rec
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.r#box)
    }
}

impl BvhNode {
    pub fn box_cmp(a: Box<dyn Hittable>, b: Box<dyn Hittable>, axis: usize) -> bool {
        let mut box_a: AABB = Default::default();
        let mut box_b: AABB = Default::default();
        let mut flag1 = false;
        let mut flag2 = false;

        if let Some(_box_a) = a.bounding_box(0., 0.) {
            box_a = _box_a;
            flag1 = true;
        }
        if let Some(_box_b) = b.bounding_box(0., 0.) {
            box_b = _box_b;
            flag2 = true;
        }

        if !flag1 || !flag2 {
            panic!("No bounding box in BvhNode constructor!");
        }

        box_a.minimum[axis] < box_b.minimum[axis]
    }
    //三个特例
    pub fn x_cmp(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        BvhNode::box_cmp(a, b, 0)
    }
    pub fn y_cmp(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        BvhNode::box_cmp(a, b, 1)
    }
    pub fn z_cmp(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        BvhNode::box_cmp(a, b, 2)
    }

    pub fn new(
        //手写的新的构造函数
        _left: Option<Box<dyn Hittable>>,
        _right: Option<Box<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        if _left.is_none() {
            panic!("BvhNode's left child is null!");
        }
        let box_left = _left.as_ref().unwrap().bounding_box(time0, time1).unwrap();
        if _right.is_some() {
            let box_right = _right.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            BvhNode {
                r#box: AABB::surrounding_box(box_left, box_right),
                left: _left,
                right: _right,
            }
        } else {
            BvhNode {
                r#box: box_left,
                left: _left,
                right: _right,
            }
        }
    }

    pub fn new_from_list(list: HittableList, time0: f64, time1: f64) -> Self {
        BvhNode::new_from_vec(list.objects, time0, time1)
    }

    pub fn new_from_vec(src_objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let mut objects = src_objects;

        let axis = random_int(0, 2) as usize;
        // 从三种比较方式中随机选择
        let cmp = |x: &Box<dyn Hittable>, y: &Box<dyn Hittable>| {
            f64::partial_cmp(
                &(x.bounding_box(time0, time1).unwrap().minimum[axis]),
                &(y.bounding_box(time0, time1).unwrap().minimum[axis]),
            )
            .unwrap()
        };

        let object_span = objects.len();
        match object_span {
            1 => {
                // 直接使用 objects[start] 赋值，需要 copy trait
                // 但是先从 vector 中出队，就生成了新的变量，所有权转移给它，不会报错
                // pop() 得到的是 Option
                let obj = objects.pop().unwrap();
                Self::new(Some(obj), None, time0, time1)
            }
            2 => {
                let obj1 = objects.pop().unwrap();
                let obj2 = objects.pop().unwrap();
                if cmp(&obj1, &obj2) == Ordering::Less {
                    Self::new(Some(obj1), Some(obj2), time0, time1)
                } else {
                    Self::new(Some(obj2), Some(obj1), time0, time1)
                }
            }
            _ => {
                objects.sort_by(cmp);
                let mut left_vec = objects;
                let right_vec = left_vec.split_off(object_span / 2);
                Self::new(
                    Some(Box::new(Self::new_from_vec(left_vec, time0, time1))),
                    Some(Box::new(Self::new_from_vec(right_vec, time0, time1))),
                    time0,
                    time1,
                )
            }
        }
    }
}
