/*
**使用了蒋捷提供的ray.rs
*/
use crate::hittables::{hit_record, hittable};
use crate::materials::{material, lambertian, metal};
pub use crate::vec3::Vec3;
use crate::Interval;
use crate::util;
use std::sync::Arc;
use crate::textures::{texture, SolidColor};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub a_origin: Vec3,
    pub b_direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(a_origin: Vec3, b_direction: Vec3, time: f64) -> Self {
        Self {
            a_origin,
            b_direction,
            time,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.a_origin + self.b_direction * t
    }
    pub fn info(&self){
        println!("ori");
        self.a_origin.info();
        println!("dir");
        self.b_direction.info();
    }
}
