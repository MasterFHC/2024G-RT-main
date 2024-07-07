/*
**使用了蒋捷提供的ray.rs
*/
use crate::hittables::{hit_record, hittable};
pub use crate::vec3::Vec3;
use crate::Interval;

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
    pub fn ray_color(&self, world: &dyn hittable) -> Vec3 {
        let mut rec: hit_record = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
        if world.hit(&self, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = self.b_direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0); //from [-1, 1], therefore mapping [0,1]
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
