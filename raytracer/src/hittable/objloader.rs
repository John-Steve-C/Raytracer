use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{objects::triangle::Triangle, HitRecord, Hittable, HittableList},
    material::{lambertian::Lambertian},
    optimization::{aabb::AABB, bvh::BvhNode}, texture::{obj::OBJTexture, image::ImageTexture},
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
    // 参考: https://docs.rs/tobj/3.2.2/tobj/struct.Mesh.html
    pub fn load_from_file(file_name: &str, pic_name : &str, t0: f64, t1: f64) -> Self
    {
        let scene = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS);

        assert!(scene.is_ok());
        let (models, mats) = scene.expect("load obj failed!");
        let _mats = mats.expect("load mtl failed!");

        let mut objects: HittableList = Default::default();

        for (i, m) in models.iter().enumerate() {
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
            // 按照 indices 的顺序读取（索引）
            for p in &mesh.indices {
                pos[cnt] = *p as usize;
                // pos[i] 记录点i 的 x 坐标的存储位置
                cnt += 1;
                if cnt == 3 {
                    let p1 = Vec3::new(
                        mesh.positions[3 * pos[0]] as f64,
                        mesh.positions[3 * pos[0] + 1] as f64,
                        mesh.positions[3 * pos[0] + 2] as f64,
                    );
                    let p2 = Vec3::new(
                        mesh.positions[3 * pos[1]] as f64,
                        mesh.positions[3 * pos[1] + 1] as f64,
                        mesh.positions[3 * pos[1] + 2] as f64,
                    );
                    let p3 = Vec3::new(
                        mesh.positions[3 * pos[2]] as f64,
                        mesh.positions[3 * pos[2] + 1] as f64,
                        mesh.positions[3 * pos[2] + 2] as f64,
                    );

                    let u = mesh.texcoords[2 * pos[0]] as f64;
                    let v = mesh.texcoords[2 * pos[0] + 1] as f64;
                    let tex = OBJTexture::new_from_file(pic_name, u, 1. - v);
                    // let tex2 = ImageTexture::new_from_file(pic_name);
                    let mat = Lambertian::new(tex);

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
