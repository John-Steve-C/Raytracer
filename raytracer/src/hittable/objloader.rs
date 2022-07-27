use crate::{
    basic_component::{ray::Ray, vec3::Vec3},
    hittable::{objects::triangle::OBJTriangle, HitRecord, Hittable, HittableList},
    material::lambertian::Lambertian,
    optimization::{aabb::AABB, bvh::BvhNode},
    texture::{image::ImageTexture, Texture},
};
// use std::sync::Arc;

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
    pub fn load_from_file(file_name: &str, t0: f64, t1: f64) -> Self {
        let scene = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS);

        assert!(scene.is_ok());
        let (models, mats) = scene.expect("load obj failed!");
        let mats = mats.expect("load mtl failed!");

        let mut objects: HittableList = Default::default();

        // 特判
        let mut is_thomas = false;
        let mut is_guy = false;
        if file_name == "import_pic/someobj/thomas.obj" {
            is_thomas = true;
        }
        if file_name == "import_pic/someobj/guy.obj" {
            is_guy = true;
        }

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
            // if mesh.material_id.is_some() {println!("material is {}!", mats[mesh.material_id.unwrap()].diffuse_texture);}

            // 特判
            let tp_index;
            if mesh.material_id.is_none() {
                tp_index = 0;
            } else {
                tp_index = mesh.material_id.unwrap();
            }

            let pic_name =
                "import_pic/someobj/".to_owned() + mats[tp_index].diffuse_texture.as_str();

            let pic_ptr = ImageTexture::new_from_file(&pic_name).image;

            // 点并不是按顺序排列的，所以不能直接读取
            let mut pos = Vec::<_>::new();
            let mut texs = Vec::<_>::new();
            // let mut normals = Vec::<_>::new();

            for p in mesh.positions.chunks(3) {
                // 以 3 为步长，进行切片（slices）
                pos.push(Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64));
            }
            for p in mesh.texcoords.chunks(2) {
                texs.push((p[0] as f64, p[1] as f64));
            }
            // for p in mesh.normals.chunks(3) {
            //     normals.push(Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64));
            // }

            // 按照 indices 的顺序读取（索引）
            for id in mesh.indices.chunks(3) {
                let mut is_al = false;
                let mut is_black = false;
                let mut is_yellow = false;

                let u =
                    (texs[id[0] as usize].0 + texs[id[1] as usize].0 + texs[id[2] as usize].0) / 3.;
                let v =
                    (texs[id[0] as usize].1 + texs[id[1] as usize].1 + texs[id[2] as usize].1) / 3.;

                // let mytex = OBJTexture::new_from_file(&pic_name, clamp(u, 0., 1.), 1. - clamp(v, 0., 1.));
                let mytex = ImageTexture {
                    image: pic_ptr.clone(),
                };

                // 特判当前颜色，用来实现金属反光贴图
                let tp_color = mytex.get_color_value(u, v, Vec3::new(0., 0., 0.));
                if is_guy && tp_color.x < 0.1 && tp_color.y < 0.1 && tp_color.z < 0.1 {
                    is_black = true;
                }
                if is_guy
                    && tp_color.x * 255. > 0.92
                    && tp_color.x < 0.93
                    && tp_color.y > 0.78
                    && tp_color.y < 0.79
                    && tp_color.z > 0.25
                    && tp_color.z < 0.26
                {
                    is_yellow = true;
                }
                if is_thomas && tp_color.x > 0.7 && tp_color.y > 0.7 && tp_color.z > 0.7 {
                    is_al = true;
                }

                let mat = Lambertian::new(mytex);
                let tri = OBJTriangle::new(
                    [
                        pos[id[0] as usize],
                        pos[id[1] as usize],
                        pos[id[2] as usize],
                    ],
                    [
                        texs[id[0] as usize],
                        texs[id[1] as usize],
                        texs[id[2] as usize],
                    ],
                    mat,
                    is_al,
                    is_black,
                    is_yellow,
                );
                // tri.normal = (normals[id[0] as usize] + normals[id[1] as usize] + normals[id[2] as usize]) / 3.;
                objects.add(tri);

                // println!("{} {} {}", id[0], id[1], id[2]);
            }
            // break; // 一次只处理一个物体
        }

        // println!("load succeed!");

        Self {
            triangles: BvhNode::new_from_list(objects, t0, t1),
        }
    }
}
