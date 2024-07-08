mod color;
mod ray;
mod vec3;
mod util;
mod sphere;
mod hittables;
mod intervals;
mod camera;
mod materials;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, hittable};
use intervals::Interval;
use camera::Camera;
use materials::{material, lambertian, metal, dielectric};
use std::rc::Rc;

fn main() {
    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 20 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(13.0, 2.0, 3.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.6;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    let ground_material = Rc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = util::random_f64_0_1();
            let center = Vec3::new(i as f64 + 0.9 * util::random_f64_0_1(), 0.2, j as f64 + 0.9 * util::random_f64_0_1());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::new(util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1());
                    sphere_material = Rc::new(lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::new(util::random_range(0.5, 1.0), util::random_range(0.5, 1.0), util::random_range(0.5, 1.0));
                    let fuzz = util::random_range(0.0, 0.5);
                    sphere_material = Rc::new(metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    sphere_material = Rc::new(dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let mut cam: Camera = Camera::new(16.0 / 9.0, 3200, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
                                        VFOV, LOOKFROM, LOOKAT, VUP,
                                        DEFOCUS_ANGLE, FOCUS_DIST);
    cam.render(world);
}
