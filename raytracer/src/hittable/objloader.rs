use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{objects::triangle::Triangle, HitRecord, Hittable, HittableList},
    material::{lambertian::Lambertian},
    optimization::{aabb::AABB, bvh::BvhNode}, texture::obj::OBJTexture,
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
        // let mats = mats.expect("load mtl failed!");

        let mut objects: HittableList = Default::default();

        // for (_i, m) in mats.iter().enumerate() {
        //     println!("i = {}", _i);
        //     println!("name : {}", m.name);
        //     println!("ambient : {:?}", m.ambient);
        //     println!("{}", m.ambient_texture);
        //     println!("diffuse : {:?}", m.diffuse);
        //     println!("{}", m.diffuse_texture);
        //     println!("specular : {:?}", m.specular);
        //     println!("{}", m.specular_texture);
        //     println!("shininess : {:?}", m.shininess);
        //     println!("{}", m.shininess_texture);
        //     println!("dissolve : {:?}", m.dissolve);
        //     println!("{}", m.dissolve_texture);
        //     println!("----------------------------------");
        // }

        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            // 查询
            // println!("mesh indices : {}", mesh.indices.len());
            // println!("positions total : {}", mesh.positions.len());
            // if mesh.vertex_color.is_empty() {println!("vertex_color is empty!");}
            // else {println!("vertex_color total : {}", mesh.vertex_color.len());}
            // if mesh.normals.is_empty() {println!("normals is empty!");}
            // else {println!("normal total : {}", mesh.normals.len());}
            // if mesh.texcoords.is_empty() {println!("texcoords is empty!");}
            // else {println!("texcoords total : {}, indices : {}", mesh.texcoords.len(), mesh.texcoord_indices.len());}
            // if mesh.material_id.is_some() {println!("material is {}!", mesh.material_id.unwrap());}


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

                    let u = (mesh.texcoords[2 * pos[0]] + mesh.texcoords[2 * pos[1]] + mesh.texcoords[2 * pos[2]]) as f64 / 3.;
                    let v = (mesh.texcoords[2 * pos[0] + 1] + mesh.texcoords[2 * pos[1] + 1] + mesh.texcoords[2 * pos[2] + 1]) as f64 / 3.;
                    
                    // println!("{}, {}", u, v);
                    let tex = OBJTexture::new_from_file(pic_name, u, 1. - v);
                    let mat = Lambertian::new(tex);

                    // let cl = Lambertian::new_from_color(Vec3::new(0.43, 0.73, 0.73));

                    objects.add(Triangle::new([p1, p2, p3], mat));
                    cnt = 0;
                }
            }
            // break;  // 一次只处理一个物体
        }

        Self {
            triangles: BvhNode::new_from_list(objects, t0, t1),
        }
    }
}
