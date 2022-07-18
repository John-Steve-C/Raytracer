use std::{
    // time::Instant,
    f64::INFINITY,
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{prelude::StdRng, Rng, SeedableRng};

pub mod basic_component;
pub mod hittable;
pub mod material;
pub mod optimization;
pub mod texture;
pub mod utility; //调用模块

use crate::{
    basic_component::{camera::Camera, ray::Ray, vec3::Vec3},
    hittable::{
        aarect::{XYRect, XZRect, YZRect},
        cube::Cube,
        flipface::Flipface,
        instance::{constant_medium::ConstantMedium, rotate::RotateY, translate::Translate},
        sphere::{MovingSphere, Sphere},
        Hittable, HittableList,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    optimization::{
        bvh::BvhNode,
        pdf::{CosinePDF, HittablePDF, PDF},
    },
    texture::{
        checker::CheckerTexture, image::ImageTexture, perlin::NoiseTexture, perlin::Perlin,
        solid::SolidColor,
    },
    utility::{get_pixel_color, random_double},
};

pub fn ray_color<T : Hittable>(
    r: Ray,
    background: Vec3,
    world: &HittableList,
    lights: &T,
    depth: i32,
) -> Vec3 {
    // 递归终止条件
    // 超出限制，光无法反射，变成黑色
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    let emitted: Vec3;
    // 判断是否碰到物体
    // t_min 修正为 0.01，因为光线并不是在 t=0 处才会击中物体
    if let Some(temp_rec) = world.hit(r, 0.001, INFINITY) {
        emitted = temp_rec
            .mat
            .emitted(r, temp_rec, temp_rec.u, temp_rec.v, temp_rec.p);

        //考虑金属的反射
        if let Some(temp_scatter) = temp_rec.mat.scatter(r, temp_rec) {
            // let p = CosinePDF::new(temp_rec.normal);
            let light_pdf = HittablePDF::new(lights, temp_rec.p);
            let scattered = Ray::new(temp_rec.p, light_pdf.generate(), r.tm);
            let pdf = light_pdf.value(scattered.dir);

            // 如果有，就是二者叠加的颜色
            emitted
                + temp_scatter.attenuation
                    * temp_rec.mat.scattering_pdf(r, temp_rec, scattered)
                    * ray_color(scattered, background, world, lights, depth - 1)
                    / pdf
        } else {
            // 金属没有反射，直接发光
            emitted
        }
    } else {
        //没碰到物体，就返回背景的颜色
        // let unit_dir = Vec3::unit_vector(r.dir);
        // let t = 0.5 * (unit_dir.y + 1.);
        // Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t //渐变色
        background
    }
}

fn scene_book2() -> HittableList {
    let mut boxes1: HittableList = Default::default();
    let mut boxes2: HittableList = Default::default();
    let mut world: HittableList = Default::default();

    let ground = Lambertian::new_from_color(Vec3::new(0.48, 0.83, 0.53));
    // 生成凹凸的地面
    let boxes_per_side = 20;
    let mut rng = StdRng::seed_from_u64(19260817); // 从特定的种子生成
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            // 伪随机，保证每次画出的图形相同
            let z1 = z0 + w;

            boxes1.add(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground,
            ));
        }
    }
    world.add(BvhNode::new_from_list(boxes1, 0., 1.));

    // 顶部的矩形光源
    let light = DiffuseLight::new_from_color(Vec3::new(7., 7., 7.));
    world.add(XZRect::new(123., 423., 147., 412., 554., light));

    let center1 = Vec3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Lambertian::new_from_color(Vec3::new(0.7, 0.3, 0.1));
    world.add(MovingSphere::new(
        50.,
        center1,
        center2,
        0.,
        1.,
        moving_sphere_material,
    ));

    world.add(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.),
    ));

    // 加上黑雾作为背景
    let mut boundary = Sphere::new(Vec3::new(360., 150., 145.), 70., Dielectric::new(1.5));
    world.add(boundary); // 额外加入一个雾化的球（恰好在中央）
    world.add(ConstantMedium::new_from_color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    ));
    boundary = Sphere::new(Vec3::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    world.add(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        Vec3::new(1., 1., 1.),
    ));

    // 地球贴图的球体
    let emat = Lambertian::new(ImageTexture::new_from_file("import_pic/earthmap.jpg"));
    world.add(Sphere::new(Vec3::new(400., 200., 400.), 100., emat));
    let pertext = NoiseTexture::new(Perlin::new(), 0.1);
    world.add(Sphere::new(
        Vec3::new(220., 280., 300.),
        80.,
        Lambertian::new(pertext),
    )); // 加上频率为 0.1 的噪声纹理，变为蓝色的球

    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73));
    let ns = 1000; //小球个数
    for _j in 0..ns {
        boxes2.add(Sphere::new(
            Vec3 {
                x: rng.gen_range(0.0..165.0),
                y: rng.gen_range(0.0..165.0),
                z: rng.gen_range(0.0..165.0),
            }, // 伪随机
            10.,
            white,
        ));
    }
    // 由小球组成的立方体，旋转+平移
    world.add(Translate::new(
        RotateY::new(BvhNode::new_from_list(boxes2, 0., 1.), 15.),
        Vec3::new(-100., 270., 395.),
    ));

    world
}

fn cornell_box() -> HittableList {
    let mut world: HittableList = Default::default();

    let red = Lambertian::new_from_color(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_from_color(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_from_color(Vec3::new(15., 15., 15.));
    // 用颜色来控制亮度？

    world.add(YZRect::new(0., 555., 0., 555., 555., green));
    world.add(YZRect::new(0., 555., 0., 555., 0., red));
    // world.add(XZRect::new(213., 343., 227., 332., 554., light));
    world.add(Flipface::new(XZRect::new(
        213., 343., 227., 332., 554., light,
    )));
    world.add(XZRect::new(0., 555., 0., 555., 0., white));
    world.add(XZRect::new(0., 555., 0., 555., 555., white));
    world.add(XYRect::new(0., 555., 0., 555., 555., white));

    // world.add(Cube::new(
    //     Vec3::new(130., 0., 65.),
    //     Vec3::new(295., 165., 230.),
    //     white,
    // ));
    // world.add(Cube::new(
    //     Vec3::new(265., 0., 295.),
    //     Vec3::new(430., 330., 460.),
    //     white,
    // ));

    let box1 = Cube::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white);
    let box2 = Cube::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white);

    // 先旋转再平移
    let rt1 = RotateY::new(box1, 15.); //旋转后的立方体 rt1
    let tr1 = Translate::new(rt1, Vec3::new(265., 0., 295.)); //平移后的立方体 tr1
    world.add(tr1);
    // 同理
    let rt2 = RotateY::new(box2, -18.);
    let tr2 = Translate::new(rt2, Vec3::new(130., 0., 65.));
    // world.add(ConstantMedium::new_from_color(
    //     tr2,
    //     0.01,
    //     Vec3::new(1., 1., 1.),
    // ));
    world.add(tr2);

    world
}

fn simple_light() -> HittableList {
    let mut world: HittableList = Default::default();

    let mat1 = Lambertian::new(NoiseTexture::new(Perlin::new(), 4.));
    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., mat1));
    world.add(Sphere::new(Vec3::new(0., 2., 0.), 2., mat1));

    let mat2 = DiffuseLight::new_from_color(Vec3::new(4., 4., 4.));
    world.add(XYRect::new(3., 5., 1., 3., -2., mat2));
    world.add(Sphere::new(Vec3::new(0., 7., 0.), 2., mat2));

    world
}

fn earth() -> HittableList {
    let mut world: HittableList = Default::default();
    let image = ImageTexture::new_from_file("import_pic/earthmap.jpg");
    let mat1 = Lambertian::new(image);

    world.add(Sphere::new(Vec3::new(0., 0., 0.), 2., mat1));

    world
}

fn two_spheres() -> HittableList {
    let mut world: HittableList = Default::default();
    // let checker = CheckerTexture {
    //     odd : SolidColor::new(0.2, 0.3, 0.1),
    //     even : SolidColor::new(0.9, 0.9, 0.9),
    // };
    let mat1 = Lambertian::new(NoiseTexture::new(Perlin::new(), 4.));

    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., mat1));
    world.add(Sphere::new(Vec3::new(0., 2., 0.), 2., mat1));

    world
}

fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let checker = CheckerTexture {
        odd: SolidColor::new(0.2, 0.3, 0.1),
        even: SolidColor::new(0.9, 0.9, 0.9),
    }; //棋盘状的纹理
    let ground_material = Lambertian::new(checker);

    world.add(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0., 1.);
            let _center = Vec3::new(
                a as f64 + random_double(0., 1.),
                0.2,
                b as f64 + random_double(0., 1.),
            );

            if (_center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let _albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    //随机生成小球的反照率
                    //用反照率对应生成'纹理'颜色
                    let sphere_material = Lambertian::new_from_color(_albedo);
                    let _center2 = _center + Vec3::new(0., random_double(0., 0.5), 0.);
                    // world.add(sphere::Sphere {
                    //     center: _center,
                    //     radius: 0.2,
                    //     mat: sphere_material,
                    // });
                    world.add(MovingSphere::new(
                        0.2,
                        _center,
                        _center2,
                        0.,
                        1.,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    //metal
                    let _albedo = Vec3::random(0.5, 1.);
                    let _fuzz = random_double(0., 0.5);
                    let sphere_material = Metal::new(_albedo, _fuzz);
                    world.add(Sphere::new(_center, 0.2, sphere_material));
                } else {
                    //glass
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(Sphere::new(_center, 0.2, sphere_material));
                }
            }
        }
    }

    let mat_1 = Dielectric::new(1.5);
    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1., mat_1));
    let mat_2 = Lambertian::new_from_color(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1., mat_2));
    let mat_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.);
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1., mat_3));

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    // ----------------------设定图像的内容-------------------------
    let aspect_ratio = 1.;
    let width = 600;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 100; // From 0 to 100
    let path = "output/output.jpg";

    let samples_per_pixel = 100;
    // 每一个像素点由多少次光线来确定
    let max_depth = 50;

    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let aperture = 0.; // 光圈，用来控制虚化
    let dist_to_focus = 10.;
    let background = Vec3::new(0., 0., 0.);
    // 黑色的背景

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    if false {
        scene_book2();
        cornell_box();
        random_scene();
        two_spheres();
        earth();
        simple_light();
    } //用来防止报错

    //------------------------------输出图像的特定信息-----------------------------
    println!(
        "Image size: {}\nJPEG quality: {}\nSamples_per_pixel: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
        style(samples_per_pixel).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);

    //-------------------------多线程部分---------------------------

    println!("Multi-threading!");
    // let begin_time = Instant::now();
    let thread_number = 16; // 线程数

    let section_line_number = height / thread_number; // 每个线程处理的行数
    let mut thread_pool = Vec::<_>::new(); // 进程池
    let mut output_pixel_color = Vec::new(); // 画出的像素颜色

    let multi_progress = Arc::new(MultiProgress::new()); // 多个进度条
    multi_progress.set_move_cursor(true);

    for thread_id in 0..thread_number {
        // 计算出行首与行末的编号
        let line_begin = section_line_number * thread_id;
        let mut line_end = line_begin + section_line_number;
        if line_end > height || (thread_id == thread_number - 1 && line_end < height) {
            // 不足的最后一行，自动补齐
            line_end = height;
        }

        // 设定图片内容
        // 要保证每次都能生成相同的图片，即部分伪随机
        let world: HittableList = cornell_box();
        // let mut lights: HittableList = Default::default();
        let light = DiffuseLight::new_from_color(Vec3::new(15., 15., 15.));
        let lights = XZRect::new(213., 343., 227., 332., 554., light);

        // 设置进度条
        let mp = multi_progress.clone();
        let progress_bar = mp.add(ProgressBar::new(((line_end - line_begin) * width) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
                                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
                                .progress_chars("#>-"));

        //-------------------- 内部的线程 -----------------------------------------------------
        let (tx, rx) = mpsc::channel(); //信道
        thread_pool.push((
            thread::spawn(move || {
                progress_bar.set_position(0);

                let channel_send = tx;
                let mut section_pixel_color = Vec::new(); // 临时记录线程的计算结果

                // 计算通过某一像素点的颜色
                for y in line_begin..line_end {
                    for x in 0..width {
                        let mut color = Vec3::new(0., 0., 0.);
                        for _s in 0..samples_per_pixel {
                            // 抗锯齿
                            let u = (x as f64 + random_double(0., 1.)) / (width - 1) as f64;
                            let v = (y as f64 + random_double(0., 1.)) / (height - 1) as f64;

                            let r = cam.get_ray(u, v); //多次求通过该像素的光线
                            color += ray_color(r, background, &world, &lights, max_depth);
                        }
                        section_pixel_color.push(color); // 记录该线程计算出的颜色

                        progress_bar.inc(1);
                    }
                }
                channel_send.send(section_pixel_color).unwrap(); // 通过信道把结果向外传递
                progress_bar.finish_with_message("Finished!");
            }),
            rx,
        ));
    }
    // 等待所有进程结束，再执行主线程
    multi_progress.join().unwrap();

    let mut thread_finish_successfully = true;
    let collecting_progress_bar = ProgressBar::new(thread_number as u64);
    // 接收信息，修改进度条
    for thread_id in 0..thread_number {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                thread_finish_successfully = false;
                println!(
                    "Joining the {} thread failed!",
                    style(thread_id.to_string()).red()
                );
            }
        }
    }
    if !thread_finish_successfully {
        exit(1);
    }
    collecting_progress_bar.finish_and_clear();

    //---------------------------利用计算结果给图像上色----------------------------------
    let mut pixel_id = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel_color = get_pixel_color(output_pixel_color[pixel_id], samples_per_pixel);
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);

            pixel_id += 1;
        }
    }

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {
            // println!(
            //     "Time used : {}",
            //     style(HumanDuration(begin_time.elapsed())).yellow()
            // );
            // 统计运行时间
        }
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
