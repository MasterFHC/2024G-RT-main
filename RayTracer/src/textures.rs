use crate::vec3::Vec3;
use std::rc::Rc;

pub trait texture {
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
    odd: Rc<dyn texture>,
    even: Rc<dyn texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Rc<dyn texture>, odd: Rc<dyn texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_from_color(scale: f64, color1: Vec3, color2: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::new(color1)),
            odd: Rc::new(SolidColor::new(color2)),
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