use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{HitRecord, Hittable, HittableList, objects::triangle::Triangle},
    material::Material,
    optimization::{aabb::AABB, bvh::BvhNode},
};

use tobj::Model;

pub struct OBJ {
    // 导入的 obj 模型由许多 三角形 组成
    pub triangles: BvhNode,
}

impl Hittable for OBJ {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.triangles.bounding_box(time0, time1)
    }
}

impl OBJ {
    pub fn load_from_file<T>(file_name: &str, mat: T, t0 : f64, t1 : f64) -> Self
    where
        T: Material + 'static + Copy,
    {
        let scene = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS);
        assert!(scene.is_ok());
        let (models, mats) = scene.expect("load obj failed!");

        let mut objects: HittableList = Default::default();

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            let mut v = 0;
            while v < mesh.positions.len() {
                let p1 = Vec3::new(mesh.positions[v] as f64, mesh.positions[v + 1] as f64, mesh.positions[v + 2] as f64);
                let p2 = Vec3::new(mesh.positions[v + 3] as f64, mesh.positions[v + 4] as f64, mesh.positions[v + 5] as f64);
                let p3 = Vec3::new(mesh.positions[v + 6] as f64, mesh.positions[v + 7] as f64, mesh.positions[v + 8] as f64);

                objects.add(Triangle::new([p1, p2, p3], mat));
                v += 9;
            }
        }

        Self { triangles: BvhNode::new_from_list(objects, t0, t1) }
    }
}
