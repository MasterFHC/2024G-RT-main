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
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Condvar;

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

        let HEIGHT_PARTITION: u32 = 20;
        let WIDTH_PARTITION: u32 = 20;
        let THREAD_LIMIT: u32 = 20;

        self.bar = if is_ci() {
            ProgressBar::hidden()
        } else {
            //ProgressBar::new((self.image_height * self.image_width) as u64)
            // ProgressBar::new((self.image_height) as u64)  //this is faster?
            ProgressBar::new((HEIGHT_PARTITION * WIDTH_PARTITION) as u64)  //this is faster???
        };
        self.bar.set_style(indicatif::ProgressStyle::default_bar()
            .template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7} {per_sec}"));
        self.bar.set_message("|0 threads outstanding|");

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let img_mtx = Arc::new(Mutex::new(&mut img));

        crossbeam::thread::scope(move |thread_spawner|{
            let thread_count = Arc::new(AtomicUsize::new(0));
            let thread_number_controller = Arc::new(Condvar::new());
      
            let chunk_height = (self.image_height + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
            let chunk_width = (self.image_width + WIDTH_PARTITION - 1) / WIDTH_PARTITION;

            let camera_wrapper = Arc::new(self);
            let world_wrapper = Arc::new(&world);

            for j in 0..HEIGHT_PARTITION {
                for i in 0..WIDTH_PARTITION {
                    // WAIT
                    let thread_count = Arc::clone(&thread_count);
                    let thread_number_controller = Arc::clone(&thread_number_controller);
                    let lock_for_condv = Mutex::new(false);
                    while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT as usize) { // outstanding thread number control
                        thread_number_controller.wait(lock_for_condv.lock().unwrap()).unwrap();
                    }
                    let camera_wrapper = Arc::clone(&camera_wrapper);
                    let world_wrapper = Arc::clone(&world_wrapper);
                    let img_mtx = Arc::clone(&img_mtx);
                    let x_min = i * chunk_width;
                    let x_max = (i + 1) * chunk_width;
                    let y_min = j * chunk_height;
                    let y_max = (j + 1) * chunk_height;
        
                    // move "thread_count++" out of child thread, so that it's sequential with thread number control code
                    thread_count.fetch_add(1, Ordering::SeqCst);
                    camera_wrapper.bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst))); // set "thread_count" information to progress bar
        
                    thread_spawner.spawn(move |_| {

                        camera_wrapper.render_sub(world, img_mtx, x_min, x_max, y_min, y_max);
        
                        thread_count.fetch_sub(1, Ordering::SeqCst); // subtract first, then notify.
                        camera_wrapper.bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst)));
                        // NOTIFY
                        thread_number_controller.notify_one();
                    });
        
                }
            }
        }).unwrap();      

        let path = "output/test.jpg";

        println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
        let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
        let mut output_file: File = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(100)) {
            Ok(_) => {}
            Err(_) => println!("Outputting image fails."),
        }
    }
    
    pub fn render_sub(&self, world: &hittable_list, img_mtx: Arc<Mutex<&mut RgbImage>>,
                        x_min: u32, x_max: u32, y_min: u32, y_max: u32) {
        
        //Render
        // println!("WIDTH: {}, HEIGHT: {}", self.image_width, self.image_height);

        //we write the colors in the buffer and then transfer them to the image to enhance performance
        // let write_buffer : [[Vec3; y_max - y_min] ; x_max - x_min];
        // let mut write_buffer : [[Vec3; 500 as usize] ; 500 as usize] = [[Vec3::zero(); 500 as usize]; 500 as usize];
        //stack overflow, i will use heap by using opencv's Mat:
        let mut write_buffer : Vec<Vec<Vec3>> = vec![vec![Vec3::zero(); (y_max - y_min) as usize]; (x_max - x_min) as usize];
 
        for j in y_min..y_max {
            for i in x_min..x_max {
                let mut pixel_color = Vec3::zero();

                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);
                    pixel_color += ray.ray_color(world, self.max_depth);
                }

                let written_color = pixel_color * (1.0 / self.samples_per_pixel as f64);
                write_buffer[(i - x_min) as usize][(j - y_min) as usize] = written_color;
                // println!("Pixel color: {:?}", written_color);
                // self.bar.inc(1);
            }
        }
        let mut binding = img_mtx.lock().unwrap();
        let img: &mut RgbImage = *binding;
        //write the colors in the write buffer to the actual image
        for j in 0..(y_max - y_min) {
            for i in 0..(x_max - x_min) {
                let writex = util::fmin(self.image_width as f64 - 1.0, (i + x_min) as f64);
                let writey = util::fmin(self.image_height as f64 - 1.0, (j + y_min) as f64);
                write_color(write_buffer[i as usize][j as usize], img, writex as usize, writey as usize);
            }
        }
        self.bar.inc(1); //this is faster?
    }
}

const AUTHOR: &str = "MasterFHC";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}