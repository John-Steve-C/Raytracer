use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

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
        instance::{constant_medium::ConstantMedium, rotate::RotateY, translate::Translate},
        r#box::Box,
        sphere::{MovingSphere, Sphere},
        HittableList,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    optimization::bvh::BvhNode,
    texture::{
        checker::CheckerTexture, image::ImageTexture, perlin::NoiseTexture, perlin::Perlin,
        solid::SolidColor,
    },
    utility::{get_pixel_color, random_double},
};

fn scene_book2() -> HittableList {
    let mut boxes1: HittableList = Default::default();
    let mut boxes2: HittableList = Default::default();
    let mut world: HittableList = Default::default();

    let ground = Lambertian::new_from_color(Vec3::new(0.48, 0.83, 0.53));
    // 生成凹凸的地面
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = random_double(1., 101.);
            let z1 = z0 + w;

            boxes1.add(Box::new(
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
    let emat = Lambertian::new(ImageTexture::new_from_file("earthmap.jpg"));
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
        boxes2.add(Sphere::new(Vec3::random(0., 165.), 10., white));
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

    world.add(YZRect::new(0., 555., 0., 555., 555., green));
    world.add(YZRect::new(0., 555., 0., 555., 0., red));
    world.add(XZRect::new(113., 443., 127., 432., 554., light));
    world.add(XZRect::new(0., 555., 0., 555., 0., white));
    world.add(XZRect::new(0., 555., 0., 555., 555., white));
    world.add(XYRect::new(0., 555., 0., 555., 555., white));

    // world.add(Box::new(
    //     Vec3::new(130., 0., 65.),
    //     Vec3::new(295., 165., 230.),
    //     white,
    // ));
    // world.add(Box::new(
    //     Vec3::new(265., 0., 295.),
    //     Vec3::new(430., 330., 460.),
    //     white,
    // ));

    let box1 = Box::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white);
    let box2 = Box::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white);

    // 先旋转再平移
    let rt1 = RotateY::new(box1, 15.); //旋转后的立方体 rt1
    let tr1 = Translate::new(rt1, Vec3::new(265., 0., 295.)); //平移后的立方体 tr1
    world.add(ConstantMedium::new_from_color(
        tr1,
        0.01,
        Vec3::new(0., 0., 0.),
    ));
    // 同理
    let rt2 = RotateY::new(box2, -18.);
    let tr2 = Translate::new(rt2, Vec3::new(130., 0., 65.));
    world.add(ConstantMedium::new_from_color(
        tr2,
        0.01,
        Vec3::new(1., 1., 1.),
    ));

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
    let image = ImageTexture::new_from_file("earthmap.jpg");
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

    let aspect_ratio = 1.;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 100; // From 0 to 100
    let path = "output/output.jpg";

    let samples_per_pixel = 200;
    let max_depth = 50;

    let lookfrom = Vec3::new(478., 278., -600.);
    let lookat = Vec3::new(278., 278., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let aperture = 0.; // 光圈，用来控制虚化
    let dist_to_focus = 10.;
    let background = Vec3::new(0., 0., 0.);

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

    let world: HittableList = scene_book2();

    if false {
        cornell_box();
        random_scene();
        two_spheres();
        earth();
        simple_light();
    } //用来防止报错

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    // Generate image
    for y in (0..height).rev() {
        for x in 0..width {
            let mut color = Vec3::new(0., 0., 0.);
            for _s in 0..samples_per_pixel {
                // 抗锯齿
                let u = (x as f64 + random_double(0., 1.)) / (width - 1) as f64;
                let v = (y as f64 + random_double(0., 1.)) / (height - 1) as f64;

                let r = cam.get_ray(u, v); //多次求通过该像素的光线
                color += Ray::ray_color(r, background, &world, max_depth);
            }

            //上色
            let pixel_color = get_pixel_color(color, samples_per_pixel);
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
