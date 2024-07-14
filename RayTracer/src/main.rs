mod color;
mod ray;
mod vec3;
mod util;
mod sphere;
mod hittables;
mod intervals;
mod camera;
mod materials;
mod aabb;
mod bvh;
mod textures;
mod perlins;

extern crate opencv;

use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, hittable};
use intervals::Interval;
use camera::Camera;
use materials::{material, lambertian, metal, dielectric};
use bvh::BVHNode;
use std::sync::Arc;
use textures::{Checker, SolidColor, Image, Noise};
use perlins::perlin;

fn bouncing_spheres() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 1600 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(13.0, 2.0, 3.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.6;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    // let ground_material = Arc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let checker = Arc::new(Checker::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_from_texture(checker)))));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = util::random_f64_0_1();
            let center = Vec3::new(i as f64 + 0.9 * util::random_f64_0_1(), 0.2, j as f64 + 0.9 * util::random_f64_0_1());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::new(util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1());
                    sphere_material = Arc::new(lambertian::new(albedo));
                    // world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));

                    //bouncy balls
                    let center2 = center + Vec3::new(0.0, util::random_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::new(util::random_range(0.5, 1.0), util::random_range(0.5, 1.0), util::random_range(0.5, 1.0));
                    let fuzz = util::random_range(0.0, 0.5);
                    sphere_material = Arc::new(metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    sphere_material = Arc::new(dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
                                        VFOV, LOOKFROM, LOOKAT, VUP,
                                        DEFOCUS_ANGLE, FOCUS_DIST);

    let mut world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn checkered_spheres() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 400 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(13.0, 2.0, 3.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    let checker = Arc::new(Checker::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Arc::new(lambertian::new_from_texture(checker.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Arc::new(lambertian::new_from_texture(checker.clone())))));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST);

    let mut world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn earth() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 400 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(12.0, 0.0, 12.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    let earth_texture = Arc::new(Image::new("double_baihua.png"));
    let earth_surface = Arc::new(lambertian::new_from_texture(earth_texture.clone()));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface)));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST);

    let mut world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn baihua() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 1600 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(13.0, 2.0, 10.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.6;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    // let ground_material = Arc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let checker = Arc::new(Checker::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_from_texture(checker)))));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = util::random_f64_0_1();
            let center = Vec3::new(i as f64 + 0.9 * util::random_f64_0_1(), 0.2, j as f64 + 0.9 * util::random_f64_0_1());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::new(util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1(), 
                                           util::random_f64_0_1() * util::random_f64_0_1());
                    // sphere_material = Arc::new(lambertian::new(albedo));
                    let earth_texture = Arc::new(Image::new("double_baihua.png"));
                    let earth_surface = Arc::new(lambertian::new_from_texture(earth_texture.clone()));

                    //bouncy balls
                    let center2 = center + Vec3::new(0.0, util::random_range(0.0, 0.01), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, earth_surface)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::new(util::random_range(0.5, 1.0), util::random_range(0.5, 1.0), util::random_range(0.5, 1.0));
                    let fuzz = util::random_range(0.0, 0.5);
                    sphere_material = Arc::new(metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    sphere_material = Arc::new(dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
                                        VFOV, LOOKFROM, LOOKAT, VUP,
                                        DEFOCUS_ANGLE, FOCUS_DIST);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn perlin_spheres() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 400 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(13.0, 2.0, 3.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let mut world = &mut (hittable_list::new());

    let pertext = Arc::new(Noise::new(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST);

    let mut world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn main() {
    // bouncing_spheres();
    // checkered_spheres();
    // earth();
    // baihua();
    perlin_spheres();
}
