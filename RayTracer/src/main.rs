mod color;
mod ray;
mod vec3;
mod util;
mod sphere;
mod hittables;
mod intervals;
// mod world;
// mod aabb;

use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, hittable};
use intervals::Interval;

const AUTHOR: &str = "MasterFHC";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    1.0
}

fn Draw(img: &mut RgbImage, aspect_ratio: f64, width: u32, height: u32, bar: &ProgressBar, world: &hittable_list) {
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    //Calculate the vectors across the horizontal and vertical axes of the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let delta_u = viewport_u / width as f64;
    let delta_v = viewport_v / height as f64;

    //Calculate the location of the top left pixel
    let viewport_upper_left = camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + (delta_u + delta_v) * 0.5;

    //Render
    println!("WIDTH: {}, HEIGHT: {}", width, height);
    for j in 0..height {
        for i in 0..width {
            let u = delta_u * i as f64;
            let v = delta_v * j as f64;
            let pixel_loc = pixel00_loc + u + v;
            let ray_dir = pixel_loc - camera_center;
            let ray = Ray::new(camera_center, ray_dir, 0.0);

            let pixel_color = ray.ray_color(world);
            write_color(pixel_color, img, i as usize, j as usize);
            bar.inc(1);
        }
    }
}

fn main() {
    let path = "output/test.jpg";

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 900;

    //Caculate image height, ensure that it's at least one pixel high
    let mut height = (width as f64 / aspect_ratio) as u32;
    if height < 1 {
        height = 1;
    }

    let quality = 100;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut world = hittable_list::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // 以下是write color和process bar的示例代码
    // let pixel_color = [255u8; 3];
    // for i in 0..100 {
    //     for j in 0..100 {
    //         write_color(pixel_color, &mut img, i, j);
    //         bar.inc(1);
    //     }
    // }
    // bar.finish();

    Draw(&mut img, aspect_ratio, width, height, &bar, &world);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
