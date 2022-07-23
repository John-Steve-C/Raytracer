use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{objects::triangle::Triangle, HitRecord, Hittable, HittableList},
    material::Material,
    optimization::{aabb::AABB, bvh::BvhNode},
};

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
    pub fn load_from_file<T>(file_name: &str, mat: T, t0: f64, t1: f64) -> Self
    where
        T: Material + 'static + Copy,
    {
        let scene = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS);

        assert!(scene.is_ok());
        let (models, mats) = scene.expect("load obj failed!");
        let mats = mats.expect("load mtl failed!");

        let mut objects: HittableList = Default::default();

        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            // let mut v = 0;
            // while v + 8 < mesh.positions.len() {
            //     let p1 = Vec3::new(mesh.positions[v] as f64, mesh.positions[v + 1] as f64, mesh.positions[v + 2] as f64);
            //     let p2 = Vec3::new(mesh.positions[v + 3] as f64, mesh.positions[v + 4] as f64, mesh.positions[v + 5] as f64);
            //     let p3 = Vec3::new(mesh.positions[v + 6] as f64, mesh.positions[v + 7] as f64, mesh.positions[v + 8] as f64);

            //     let tri = Triangle::new([p1, p2, p3], mat);
            //     objects.add(tri);
            //     v += 9;
            // } 错误示范

            // 点并不是按顺序排列的，所以不能直接读取
            let mut cnt = 0;
            let mut pos = [0; 3];
            // 按照 indices 的顺序读取
            for p in &mesh.indices {
                pos[cnt] = (*p as usize) * 3;
                // pos[i] 记录点i 的 x 坐标
                cnt += 1;
                if cnt == 3 {
                    let p1 = Vec3::new(
                        mesh.positions[pos[0]] as f64,
                        mesh.positions[pos[0] + 1] as f64,
                        mesh.positions[pos[0] + 2] as f64,
                    );
                    let p2 = Vec3::new(
                        mesh.positions[pos[1]] as f64,
                        mesh.positions[pos[1] + 1] as f64,
                        mesh.positions[pos[1] + 2] as f64,
                    );
                    let p3 = Vec3::new(
                        mesh.positions[pos[2]] as f64,
                        mesh.positions[pos[2] + 1] as f64,
                        mesh.positions[pos[2] + 2] as f64,
                    );
                    objects.add(Triangle::new([p1, p2, p3], mat));
                    cnt = 0;
                }
            }
            // break;  // 一次只处理一个物体
        }

        // for (_i, m) in mats.iter().enumerate() {
        //     m.ambient[0]
        // }

        Self {
            triangles: BvhNode::new_from_list(objects, t0, t1),
        }
    }
}
