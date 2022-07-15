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
        r#box::Box,
        sphere::{MovingSphere, Sphere},
        HittableList,
        instance::{rotate::RotateY, translate::Translate},
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::{
        checker::CheckerTexture, image::ImageTexture, perlin::NoiseTexture, perlin::Perlin,
        solid::SolidColor,
    },
    utility::{get_pixel_color, random_double},
};

fn cornell_box() -> HittableList {
    let mut world: HittableList = Default::default();

    let red = Lambertian {
        albedo: SolidColor {
            color_value: Vec3::new(0.65, 0.05, 0.05),
        },
    };
    let white = Lambertian {
        albedo: SolidColor {
            color_value: Vec3::new(0.73, 0.73, 0.73),
        },
    };
    let green = Lambertian {
        albedo: SolidColor {
            color_value: Vec3::new(0.12, 0.45, 0.15),
        },
    };
    let light = DiffuseLight {
        emit: SolidColor {
            color_value: Vec3::new(15., 15., 15.),
        },
    };

    world.add(YZRect {
        y0: 0.,
        y1: 555.,
        z0: 0.,
        z1: 555.,
        k: 555.,
        mp: green,
    });
    world.add(YZRect {
        y0: 0.,
        y1: 555.,
        z0: 0.,
        z1: 555.,
        k: 0.,
        mp: red,
    });
    world.add(XZRect {
        x0: 213.,
        x1: 343.,
        z0: 227.,
        z1: 332.,
        k: 554.,
        mp: light,
    });
    world.add(XZRect {
        x0: 0.,
        x1: 555.,
        z0: 0.,
        z1: 555.,
        k: 0.,
        mp: white,
    });
    world.add(XZRect {
        x0: 0.,
        x1: 555.,
        z0: 0.,
        z1: 555.,
        k: 555.,
        mp: white,
    });
    world.add(XYRect {
        x0: 0.,
        x1: 555.,
        y0: 0.,
        y1: 555.,
        k: 555.,
        mp: white,
    });

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

    let box1 = Box::new(
        Vec3::new(0., 0., 0.), 
        Vec3::new(165., 330., 165.), 
        white
    );
    let box2 = Box::new(
        Vec3::new(0., 0., 0.), 
        Vec3::new(165., 165., 165.), 
        white
    );

    let rt1 = RotateY::new(box1, 15.);
    let tr1 = Translate{
        after_box : rt1,
        offset : Vec3::new(265., 0., 295.),
    };
    world.add(tr1);

    let rt2 = RotateY::new(box2, -18.);
    let tr2 = Translate{
        after_box : rt2,
        offset : Vec3::new(130., 0., 65.),
    };
    world.add(tr2);

    world
}

fn simple_light() -> HittableList {
    let mut world: HittableList = Default::default();

    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.,
    };
    let mat1 = Lambertian { albedo: pertext };
    world.add(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        mat: mat1,
    });
    world.add(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        mat: mat1,
    });

    let solidtext = SolidColor::new(4., 4., 4.);
    let mat2 = DiffuseLight { emit: solidtext };
    world.add(XYRect {
        x0: 3.,
        x1: 5.,
        y0: 1.,
        y1: 3.,
        k: -2.,
        mp: mat2,
    });
    world.add(Sphere {
        center: Vec3::new(0., 7., 0.),
        radius: 2.,
        mat: mat2,
    });

    world
}

fn earth() -> HittableList {
    let mut world: HittableList = Default::default();
    let image = ImageTexture::new_from_file("earthmap.jpg");
    let mat1 = Lambertian { albedo: image };

    world.add(Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 2.,
        mat: mat1,
    });

    world
}

fn two_spheres() -> HittableList {
    let mut world: HittableList = Default::default();
    // let checker = CheckerTexture {
    //     odd : SolidColor::new(0.2, 0.3, 0.1),
    //     even : SolidColor::new(0.9, 0.9, 0.9),
    // };
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.,
    };
    let mat1 = Lambertian { albedo: pertext };

    world.add(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        mat: mat1,
    });
    world.add(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        mat: mat1,
    });

    world
}

fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let checker = CheckerTexture {
        odd: SolidColor::new(0.2, 0.3, 0.1),
        even: SolidColor::new(0.9, 0.9, 0.9),
    }; //棋盘状的纹理
    let ground_material = Lambertian { albedo: checker };

    world.add(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        mat: ground_material,
    });

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
                    let __albedo: SolidColor = SolidColor {
                        color_value: _albedo,
                    };
                    //用反照率对应生成'纹理'颜色
                    let sphere_material = Lambertian { albedo: __albedo };
                    let _center2 = _center + Vec3::new(0., random_double(0., 0.5), 0.);
                    // world.add(sphere::Sphere {
                    //     center: _center,
                    //     radius: 0.2,
                    //     mat: sphere_material,
                    // });
                    world.add(MovingSphere {
                        center0: _center,
                        center1: _center2,
                        time0: 0.,
                        time1: 1.,
                        radius: 0.2,
                        mat: sphere_material,
                    });
                } else if choose_mat < 0.95 {
                    //metal
                    let _albedo = Vec3::random(0.5, 1.);
                    let _fuzz = random_double(0., 0.5);
                    let sphere_material = Metal {
                        albedo: _albedo,
                        fuzz: _fuzz,
                    };
                    world.add(Sphere {
                        center: _center,
                        radius: 0.2,
                        mat: sphere_material,
                    });
                } else {
                    //glass
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(Sphere {
                        center: _center,
                        radius: 0.2,
                        mat: sphere_material,
                    });
                }
            }
        }
    }

    let mat_1 = Dielectric { ir: 1.5 };
    world.add(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        mat: mat_1,
    });

    let mat_2 = Lambertian {
        albedo: SolidColor::new(0.4, 0.2, 0.1),
    };
    world.add(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        mat: mat_2,
    });

    let mat_3 = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    };
    world.add(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        mat: mat_3,
    });

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let aspect_ratio = 1.;
    let width = 600;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 100; // From 0 to 100
    let path = "output/output.jpg";

    let samples_per_pixel = 200;
    let max_depth = 50;

    let lookfrom = Vec3::new(278., 278., -800.);
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

    let world: HittableList = cornell_box();

    if false {
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
