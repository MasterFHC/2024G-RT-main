mod color;
mod ray;
mod vec3;
mod util;
mod sphere;
mod hittables;
mod intervals;
mod camera;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, hittable};
use intervals::Interval;
use camera::Camera;

fn main() {
    let mut world = &mut (hittable_list::new());
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam: Camera = Camera::new(16.0 / 9.0, 800, 100 as u8, 50, 10);
    cam.render(world);
}
