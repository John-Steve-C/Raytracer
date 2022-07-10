use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub mod basic_component;
pub mod hittable;
pub mod material;
pub mod utility; //调用模块

use crate::{
    basic_component::{camera::Camera, ray::Ray, vec3::Vec3},
    hittable::{sphere, HittableList},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    utility::{get_pixel_color, random_double},
};

fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let ground_material = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };

    world.add(sphere::Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        mat: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0., 1.);
            let center = Vec3::new(
                a as f64 + random_double(0., 1.),
                0.2,
                b as f64 + random_double(0., 1.),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let _albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    let sphere_material = Lambertian { albedo: _albedo };
                    world.add(sphere::Sphere {
                        center: center,
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
                    world.add(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        mat: sphere_material,
                    });
                } else {
                    //glass
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        mat: sphere_material,
                    });
                }
            }
        }
    }

    let mat_1 = Dielectric { ir: 1.5 };
    world.add(sphere::Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        mat: mat_1,
    });

    let mat_2 = Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.add(sphere::Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        mat: mat_2,
    });

    let mat_3 = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    };
    world.add(sphere::Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        mat: mat_3,
    });

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let aspect_ratio = 3. / 2.;
    let width = 1200;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 100; // From 0 to 100
    let path = "output/output.jpg";

    let samples_per_pixel = 500;
    let max_depth = 50;

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let aperture = 0.1;
    let dist_to_focus = 10.;

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world: HittableList = random_scene();

    // let material_ground = Lambertian {
    //     albedo: Vec3::new(0.8, 0.8, 0.),
    // };
    // let material_center = Lambertian {
    //     albedo: Vec3::new(0.1, 0.2, 0.5)
    // };
    // let material_left = Dielectric {
    //     ir: 1.5,
    // };
    // let material_right = Metal {
    //     albedo: Vec3::new(0.8, 0.6, 0.2),
    //     fuzz: 0.
    // };

    // world.add(sphere::Sphere {
    //     center: Vec3::new(0., -100.5, -1.),
    //     radius: 100.,
    //     mat: material_ground,
    // });
    // world.add(sphere::Sphere {
    //     center: Vec3::new(0., 0., -1.),
    //     radius: 0.5,
    //     mat: material_center,
    // });
    // world.add(sphere::Sphere {
    //     center: Vec3::new(-1., 0., -1.),
    //     radius: 0.5,
    //     mat: material_left,
    // });
    // world.add(sphere::Sphere {
    //     center: Vec3::new(-1., 0., -1.),
    //     radius: -0.45,
    //     mat: material_left,
    // });
    // world.add(sphere::Sphere {
    //     center: Vec3::new(1., 0., -1.),
    //     radius: 0.5,
    //     mat: material_right,
    // });

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
                color += Ray::ray_color(r, &world, max_depth);
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
