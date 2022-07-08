use crate::{
    ray::Ray,
    vec3::Vec3
};

#[derive(Copy, Clone, Default)]
pub struct HitRecord{
    pub p : Vec3,
    pub normal : Vec3, //法向量
    pub t : f64,
    pub front_face : bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r : Ray, outward_normal : Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.;
        if self.front_face == true {
            self.normal = outward_normal;
        } else {
            self.normal = Vec3::new(0., 0., 0.) - outward_normal;
        }
    }
}
pub trait Hittable {//特性，用于实现继承
    //代替c++中包裹record的类
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
    //优化，用 Option 是否为 None 来判断碰撞与否，同时包括返回值
}

//------------------------------------

#[derive(Default)]
pub struct HittableList{
    pub objects : Vec<Box<dyn Hittable>>    
}

impl HittableList{
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T>(&mut self, obj : T) 
    where T : Hittable + 'static 
    {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList{
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord>{
        let mut hit_rec : Option<HitRecord> = Option::None;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            if let Some(temp_rec) = i.hit(r, t_min, closest_so_far) {
                //此处的 temp_rec 相当于 something，只是表示不为 None
                closest_so_far = temp_rec.t;
                hit_rec = Some(temp_rec);
            }
        }

        hit_rec
    }
}
