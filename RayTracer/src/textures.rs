use crate::vec3::Vec3;
use std::sync::Arc;
use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub trait texture : Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo,
        }
    }
}

impl texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct Checker {
    inv_scale: f64,
    odd: Arc<dyn texture + Send + Sync>,
    even: Arc<dyn texture + Send + Sync>,
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn texture>, odd: Arc<dyn texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_from_color(scale: f64, color1: Vec3, color2: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(color1)),
            odd: Arc::new(SolidColor::new(color2)),
        }
    }
}

impl texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let xInteger = (self.inv_scale * p.x()).floor() as i32;
        let yInteger = (self.inv_scale * p.y()).floor() as i32;
        let zInteger = (self.inv_scale * p.z()).floor() as i32;
        let is_even = (xInteger + yInteger + zInteger) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct Image {
    pub img_data: opencv::core::Mat,
    width: u32,
    height: u32,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn new(filename: &str) -> Self {
        let img_data = imread(&("./mytexture/".to_owned() + filename), IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as u32;
        let height = img_data.rows() as u32;
        Self {
            img_data,
            width,
            height,
        }
    }
    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vec3 {
        // println!("u: {}, v: {}", u, v);
        if u <= 0.0 { u = 0.001; }
        if u >= 1.0 { u = 0.999; }
        if v <= 0.0 { v = 0.001; }
        if v >= 1.0 { v = 0.999; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();
        // println!("color: {:?}", color);

        Vec3::new(color[2] as f64, color[1] as f64, color[0] as f64) * (1.0 / 255.0)
    }
}

impl texture for Image {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if self.width == 0 || self.height == 0 {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        let org_color = self.get_color(u, v);

        //Adjust the color to right gamma
        Vec3::new(
            org_color.x * org_color.x(),
            org_color.y * org_color.y(),
            org_color.z * org_color.z(),
        )
    }
}