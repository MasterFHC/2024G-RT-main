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
use crate::materials::{material, lambertian, metal};

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

    //positionable camera
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    u: Vec3, // basis vectors
    v: Vec3,
    w: Vec3,

    //defocus blur
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, quality: u8, samples_per_pixel: u32, max_depth: u32, 
            vfov: f64, lookfrom: Vec3, lookat: Vec3, vup: Vec3,
            defocus_angle: f64, focus_dist: f64) -> Self {
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

            //positionable camera
            vfov,
            lookfrom,
            lookat,
            vup,
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),

            //defocus blur
            defocus_angle,
            focus_dist,
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
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
            //ProgressBar::new((self.image_height * self.image_width) as u64)
            ProgressBar::new((self.image_height) as u64)  //this is faster
        };


        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = self.aspect_ratio * viewport_height;
        self.camera_center = self.lookfrom;

        //Calculate the basis vectors
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        //Calculate the vectors across the horizontal and vertical axes of the viewport
        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * viewport_height * (-1.0);

        self.delta_u = viewport_u / self.image_width as f64;
        self.delta_v = viewport_v / self.image_height as f64;

        //Calculate the location of the top left pixel
        let viewport_upper_left = self.camera_center - viewport_u / 2.0 - viewport_v / 2.0 - self.w * self.focus_dist;
        self.pixel00_loc = viewport_upper_left + (self.delta_u + self.delta_v) * 0.5;

        //Calculate the defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = util::random_vec3() * 0.5;
        let pixel_loc = self.pixel00_loc 
                        + self.delta_u * (u + offset.x)  
                        + self.delta_v * (v + offset.y);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            let p = util::random_in_unit_disk();
            let disk_offset = self.defocus_disk_u * p.x + self.defocus_disk_v * p.y;
            self.camera_center + disk_offset
        };
        let ray_dir = pixel_loc - ray_origin;

        let ray_time = util::random_f64_0_1();
        Ray::new(ray_origin, ray_dir, ray_time)
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

                let written_color = pixel_color * (1.0 / self.samples_per_pixel as f64);
                // println!("Pixel color: {:?}", written_color);
                write_color(written_color, &mut img, i as usize, j as usize);
                // self.bar.inc(1);
            }
            self.bar.inc(1); //this is faster
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