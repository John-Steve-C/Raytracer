use std::{fs::File, io::BufReader};

use crate::{optimization::{bvh::BvhNode, aabb::AABB}, basic_component::{vec3::Vec3, ray::Ray}, material::Material};

use super::{HittableList, objects::triangle::Triangle, Hittable};

pub struct STL{
    pub triangles : BvhNode,
}

impl Hittable for STL {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        self.triangles.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.triangles.bounding_box(time0, time1)
    }
}

impl STL {
    pub fn load_from_file<T>(file_name: &str, t0: f64, t1: f64, _mat : T) -> Self
    where
        T : Material + 'static + Copy
    {
        let file = File::open(file_name).unwrap();
        let mut ast = BufReader::new(&file);
        let mesh : nom_stl::Mesh = nom_stl::parse_stl(&mut ast).unwrap();

        let mut objects : HittableList = Default::default();
        
        for &obj in mesh.triangles() {
            let x = Vec3::new(obj.vertices()[0][0] as f64, obj.vertices()[0][1] as f64, obj.vertices()[0][2] as f64);
            let y = Vec3::new(obj.vertices()[1][0] as f64, obj.vertices()[1][1] as f64, obj.vertices()[1][2] as f64);
            let z = Vec3::new(obj.vertices()[2][0] as f64, obj.vertices()[2][1] as f64, obj.vertices()[2][2] as f64);

            objects.add(Triangle::new([x, y, z], _mat));
        }

        Self{
            triangles: BvhNode::new_from_list(objects, t0, t1),
        }
    }
}