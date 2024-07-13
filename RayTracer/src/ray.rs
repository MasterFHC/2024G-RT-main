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
    pub fn ray_color(&self, world: &dyn hittable, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        let mut rec: hit_record = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            // mat: Arc::new(lambertian { tex: Arc::new(SolidColor::new(Vec3::zero())) }),
            mat: Arc::new(lambertian::new_from_texture(Arc::new(SolidColor::new(Vec3::zero())))),
            u: 0.0,
            v: 0.0,
        };
        if world.hit(&self, &mut Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
            let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
            // println!("got u: {}, got v: {}", rec.u, rec.v);
            if rec.mat.scatter(&self, &rec, &mut attenuation, &mut scattered) {
                let new_ray_color = scattered.ray_color(world, depth - 1);
                return Vec3::new(
                    attenuation.x() * new_ray_color.x(),
                    attenuation.y() * new_ray_color.y(),
                    attenuation.z() * new_ray_color.z(),
                );
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = self.b_direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0); //from [-1, 1], therefore mapping [0,1]
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
