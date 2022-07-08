use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    ray::Ray,
    vec3::Vec3,
    hittable::HittableList
};
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3; //调用模块

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 225;
    let width = 400;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";

    let view_height = 2.;
    let view_width = 32. / 9.;
    let focal_length = 1.;
    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(view_width, 0., 0.);
    let vertical = Vec3::new(0., view_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    let mut world: HittableList = Default::default();
    world.add(sphere::Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    });

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
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;

            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u + vertical * v,
            };
            let color = Ray::ray_color(r, &world);

            let pixel_color = [
                (color.x * 255.).floor() as u8,
                (color.y * 255.).floor() as u8,
                (color.z * 255.).floor() as u8,
            ];
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
