use crate::color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, hittable};
use crate::intervals::Interval;
use crate::util;

pub struct Camera {
    //basic camera settings
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub quality: u8,
    bar: ProgressBar,
    image_height: u32,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,

    //anti-aliasing
    pub samples_per_pixel: u32,

    //avoid too much recursion
    pub max_depth: u32,

}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, quality: u8, samples_per_pixel: u32, max_depth: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            quality,
            bar: ProgressBar::hidden(),
            image_height: 0,
            camera_center: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
            delta_u: Vec3::zero(),
            delta_v: Vec3::zero(),

            //anti-aliasing
            samples_per_pixel,

            //avoid too much recursion
            max_depth,
        }
    }
    fn initialize(&mut self) {
        //Caculate image height, ensure that it's at least one pixel high
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.bar = if is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        self.camera_center = Vec3::new(0.0, 0.0, 0.0);

        //Calculate the vectors across the horizontal and vertical axes of the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.delta_u = viewport_u / self.image_width as f64;
        self.delta_v = viewport_v / self.image_height as f64;

        //Calculate the location of the top left pixel
        let viewport_upper_left = self.camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        self.pixel00_loc = viewport_upper_left + (self.delta_u + self.delta_v) * 0.5;
    }
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = util::random_vec3() * 0.5;
        let pixel_loc = self.pixel00_loc 
                        + self.delta_u * (u + offset.x)  
                        + self.delta_v * (v + offset.y);
        let ray_origin = self.camera_center;
        let ray_dir = pixel_loc - self.camera_center;

        Ray::new(ray_origin, ray_dir, 0.0)
    }

    pub fn render(&mut self, world: &hittable_list) {
        self.initialize();

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        
        //Render
        println!("WIDTH: {}, HEIGHT: {}", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::zero();

                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);
                    pixel_color += ray.ray_color(world, self.max_depth);
                }
                write_color(pixel_color * (1.0 / self.samples_per_pixel as f64), &mut img, i as usize, j as usize);
                self.bar.inc(1);
            }
        }

        let path = "output/test.jpg";

        println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
        let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
        let mut output_file: File = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(self.quality)) {
            Ok(_) => {}
            Err(_) => println!("Outputting image fails."),
        }
    }
}

const AUTHOR: &str = "MasterFHC";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}