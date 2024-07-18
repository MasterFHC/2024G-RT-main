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
mod quads;

extern crate opencv;

use vec3::Vec3;
use sphere::Sphere;
use crate::hittables::{hit_record, hittable_list, translate, rotate_y, constant_medium};
use intervals::Interval;
use camera::Camera;
use materials::{material, lambertian, metal, dielectric, diffuse_light, isotropic};
use bvh::BVHNode;
use std::sync::Arc;
use textures::{Checker, SolidColor, Image, Noise};
use quads::{quad, newbox};

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

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

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
                                        DEFOCUS_ANGLE, FOCUS_DIST,
                                        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
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

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

    let checker = Arc::new(Checker::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Arc::new(lambertian::new_from_texture(checker.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Arc::new(lambertian::new_from_texture(checker.clone())))));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
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

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

    let earth_texture = Arc::new(Image::new("double_baihua.png"));
    let earth_surface = Arc::new(lambertian::new_from_texture(earth_texture.clone()));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface)));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
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

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

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
                                        DEFOCUS_ANGLE, FOCUS_DIST,
                                        BACKGROUND);

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

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

    let pertext = Arc::new(Noise::new(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn quads() {
    let ASPECT_RATIO = 1.0 as f64;
    let IMAGE_WIDTH = 400 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 80.0 as f64;

    let LOOKFROM = Vec3::new(0.0, 0.0, 9.0);
    let LOOKAT = Vec3::new(0.0, 0.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.7, 0.8, 1.0);

    let world = &mut (hittable_list::new());

    let left_red = Arc::new(lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(lambertian::new(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(lambertian::new(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(lambertian::new(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(lambertian::new(Vec3::new(0.2, 0.8, 0.8)));

    let sphere_material = Arc::new(dielectric::new(1.5));
    
    world.add(Arc::new(quad::new(Vec3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Arc::new(quad::new(Vec3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Arc::new(quad::new(Vec3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Arc::new(quad::new(Vec3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Arc::new(quad::new(Vec3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal)));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn simple_light() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 400 as u32;

    let SAMPLES_PER_PIXEL = 100 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 20.0 as f64;

    let LOOKFROM = Vec3::new(26.0, 3.0, 6.0);
    let LOOKAT = Vec3::new(0.0, 2.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let pertext = Arc::new(Noise::new(4.0)); 
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));
    
    let difflight = Arc::new(diffuse_light::new_from_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    world.add(Arc::new(quad::new(Vec3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight.clone())));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn cornell_box() {
    let ASPECT_RATIO = 1.0 as f64;
    let IMAGE_WIDTH = 600 as u32;

    let SAMPLES_PER_PIXEL = 200 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 40.0 as f64;

    let LOOKFROM = Vec3::new(278.0, 278.0, -800.0);
    let LOOKAT = Vec3::new(278.0, 278.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let red = Arc::new(lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(diffuse_light::new_from_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red.clone())));
    world.add(Arc::new(quad::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = newbox(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box2 = newbox(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone());

    let box1 = rotate_y::new(Arc::new(box1), 15.0);
    let box2 = rotate_y::new(Arc::new(box2), -18.0);

    let box1 = translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let box2 = translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(Arc::new(box1));
    world.add(Arc::new(box2));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn cornell_smoke() {
    let ASPECT_RATIO = 1.0 as f64;
    let IMAGE_WIDTH = 600 as u32;

    let SAMPLES_PER_PIXEL = 200 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 40.0 as f64;

    let LOOKFROM = Vec3::new(278.0, 278.0, -800.0);
    let LOOKAT = Vec3::new(278.0, 278.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let red = Arc::new(lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(diffuse_light::new_from_color(Vec3::new(7.0, 7.0, 7.0)));

    //The Cornell Box
    world.add(Arc::new(quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red.clone())));
    world.add(Arc::new(quad::new(Vec3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), light.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = newbox(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box2 = newbox(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone());

    let box1 = rotate_y::new(Arc::new(box1), 15.0);
    let box2 = rotate_y::new(Arc::new(box2), -18.0);

    let box1 = translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let box2 = translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(Arc::new(constant_medium::new(Arc::new(box1), 0.01, Vec3::new(0.0, 0.0, 0.0))));
    world.add(Arc::new(constant_medium::new(Arc::new(box2), 0.01, Vec3::new(1.0, 1.0, 1.0))));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let ASPECT_RATIO = 1.0 as f64;
    let IMAGE_WIDTH = image_width as u32;

    let SAMPLES_PER_PIXEL = samples_per_pixel as u32;
    let MAX_DEPTH = max_depth as u32;
    let VFOV = 40.0 as f64;

    let LOOKFROM = Vec3::new(478.0, 278.0, -600.0);
    let LOOKAT = Vec3::new(278.0, 278.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let boxes1 = &mut (hittable_list::new());
    let ground = Arc::new(lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let BOXES_PER_SIDE = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = util::random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(newbox(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }

    world.add(Arc::new(BVHNode::new_from_list(boxes1)));

    let light = Arc::new(diffuse_light::new_from_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(quad::new(Vec3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light.clone())));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(center1, center2, 50.0, moving_sphere_material.clone())));

    world.add(Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::new(dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::new(metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::new(dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(constant_medium::new(boundary.clone(), 0.2, Vec3::new(0.2, 0.4, 0.9))));
    let boundary = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(dielectric::new(1.5))));
    world.add(Arc::new(constant_medium::new(boundary.clone(), 0.0001, Vec3::new(1.0, 1.0, 1.0))));

    //NOW MAKE A EARTH
    let earth_texture = Arc::new(Image::new("earthmap.jpg"));
    let earth_surface = Arc::new(lambertian::new_from_texture(earth_texture.clone()));
    world.add(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earth_surface.clone())));
    let pertext = Arc::new(Noise::new(0.2));
    world.add(Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));

    let boxes2 = &mut (hittable_list::new());
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for i in 0..ns {
        boxes2.add(Arc::new(Sphere::new(Vec3::new(util::random_range_int(0, 165) as f64, 
                        util::random_range_int(0, 165) as f64, util::random_range_int(0, 165) as f64), 10.0, white.clone())));
    }

    world.add(Arc::new(translate::new(
                Arc::new(rotate_y::new(
                    Arc::new(BVHNode::new_from_list(boxes2)), 15.0
                )), Vec3::new(-100.0, 270.0, 395.0)
            )
        ));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}
fn final_scene_mod(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let ASPECT_RATIO = 1.0 as f64;
    let IMAGE_WIDTH = image_width as u32;

    let SAMPLES_PER_PIXEL = samples_per_pixel as u32;
    let MAX_DEPTH = max_depth as u32;
    let VFOV = 40.0 as f64;

    let LOOKFROM = Vec3::new(478.0, 278.0, -600.0);
    let LOOKAT = Vec3::new(278.0, 278.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let boxes1 = &mut (hittable_list::new());
    let ground = Arc::new(lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let BOXES_PER_SIDE = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = util::random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(newbox(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }

    world.add(Arc::new(BVHNode::new_from_list(boxes1)));

    let light = Arc::new(diffuse_light::new_from_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(quad::new(Vec3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light.clone())));

    let ljn_texture = Arc::new(Image::new("ljn_red.png"));
    let ljn_surface = Arc::new(isotropic::new_from_texture(ljn_texture.clone()));
    world.add(Arc::new(quad::new(Vec3::new(123.0, 544.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), ljn_surface.clone())));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(center1, center2, 50.0, moving_sphere_material.clone())));

    world.add(Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::new(dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::new(metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::new(dielectric::new(1.5))));
    world.add(boundary.clone());
    // world.add(Arc::new(constant_medium::new(boundary.clone(), 0.2, Vec3::new(0.2, 0.4, 0.9))));
    world.add(Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 60.0, light.clone())));
    let boundary = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(dielectric::new(1.5))));
    world.add(Arc::new(constant_medium::new(boundary.clone(), 0.0001, Vec3::new(1.0, 1.0, 1.0))));

    //NOW MAKE A EARTH
    let earth_texture = Arc::new(Image::new("double_baihua_aligned.png"));
    let earth_surface = Arc::new(lambertian::new_from_texture(earth_texture.clone()));
    world.add(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earth_surface.clone())));
    let pertext = Arc::new(Noise::new(0.2));
    world.add(Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(lambertian::new_from_texture(pertext.clone())))));

    let boxes2 = &mut (hittable_list::new());
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for i in 0..ns {
        boxes2.add(Arc::new(Sphere::new(Vec3::new(util::random_range_int(0, 165) as f64, 
                        util::random_range_int(0, 165) as f64, util::random_range_int(0, 165) as f64), 10.0, white.clone())));
    }

    world.add(Arc::new(translate::new(
                Arc::new(rotate_y::new(
                    Arc::new(BVHNode::new_from_list(boxes2)), 15.0
                )), Vec3::new(-100.0, 270.0, 395.0)
            )
        ));

    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}

fn minecraft() {
    let ASPECT_RATIO = 16.0 / 9.0 as f64;
    let IMAGE_WIDTH = 1600 as u32;

    let SAMPLES_PER_PIXEL = 500 as u32;
    let MAX_DEPTH = 50 as u32;
    let VFOV = 40.0 as f64;

    let LOOKFROM = Vec3::new(-10.0, 15.0, -10.0);
    let LOOKAT = Vec3::new(0.0, 4.0, 0.0);
    let VUP = Vec3::new(0.0, 1.0, 0.0);

    let DEFOCUS_ANGLE = 0.0;
    let FOCUS_DIST = 10.0;

    let BACKGROUND = Vec3::new(0.0, 0.0, 0.0);

    let world = &mut (hittable_list::new());

    let dirt_tex = Arc::new(Image::new("dirt.png"));
    let dirt = Arc::new(lambertian::new_from_texture(dirt_tex.clone()));

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                world.add(Arc::new(newbox(Vec3::new(i as f64, j as f64, k as f64), Vec3::new(i as f64 + 1.0, j as f64 + 1.0, k as f64 + 1.0), dirt.clone())));
            }
        }
    }

    for i in 3..6 {
        for j in 0..3 {
            for k in 0..3 {
                world.add(Arc::new(newbox(Vec3::new(i as f64, j as f64, k as f64), Vec3::new(i as f64 + 1.0, j as f64 + 1.0, k as f64 + 1.0), dirt.clone())));
            }
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            for k in 3..6 {
                world.add(Arc::new(newbox(Vec3::new(i as f64, j as f64, k as f64), Vec3::new(i as f64 + 1.0, j as f64 + 1.0, k as f64 + 1.0), dirt.clone())));
            }
        }
    }

    let oak_tex = Arc::new(Image::new("oak_log.png"));
    let oak = Arc::new(lambertian::new_from_texture(oak_tex.clone()));
    for j in 3..7 {
        let i = 1.0 as f64;
        let k = 4.0 as f64;
        world.add(Arc::new(newbox(Vec3::new(i, j as f64, k), Vec3::new(i + 1.0, j as f64 + 1.0, k + 1.0), oak.clone())));
    }

    let oak_leaves_tex = Arc::new(Image::new("oak_leaves.png"));
    let oak_leaves = Arc::new(lambertian::new_from_texture(oak_leaves_tex.clone()));
    for i in -1..4 {
        for k in 2..7 {
            let j = 6.0 as f64;
            if (i == -1 && k == 2) || (i == 3 && k == 2) || (i == -1 && k == 6) || (i == 3 && k == 6) {
                continue;
            }
            if i == 1 && k == 4 {
                continue;
            }
            world.add(Arc::new(newbox(Vec3::new(i as f64, j, k as f64), Vec3::new(i as f64 + 1.0, j + 1.0, k as f64 + 1.0), oak_leaves.clone())));
        }
    }

    for i in -1..4 {
        for k in 2..7 {
            let j = 7.0 as f64;
            if (i == -1 && k == 2) || (i == 3 && k == 2) || (i == -1 && k == 6) || (i == 3 && k == 6) {
                continue;
            }
            world.add(Arc::new(newbox(Vec3::new(i as f64, j, k as f64), Vec3::new(i as f64 + 1.0, j + 1.0, k as f64 + 1.0), oak_leaves.clone())));
        }
    }

    for i in 0..3 {
        for k in 3..6 {
            let j = 8.0 as f64;
            world.add(Arc::new(newbox(Vec3::new(i as f64, j, k as f64), Vec3::new(i as f64 + 1.0, j + 1.0, k as f64 + 1.0), oak_leaves.clone())));
        }
    }

    for i in 0..3 {
        for k in 3..6 {
            let j = 9.0 as f64;
            if (i == 0 && k == 3) || (i == 2 && k == 3) || (i == 0 && k == 5) || (i == 2 && k == 5) {
                continue;
            }
            world.add(Arc::new(newbox(Vec3::new(i as f64, j, k as f64), Vec3::new(i as f64 + 1.0, j + 1.0, k as f64 + 1.0), oak_leaves.clone())));
        }
    }

    world.add(Arc::new(newbox(Vec3::new(5.0, 3.0, 1.0), Vec3::new(6.0, 4.0, 2.0), oak_leaves.clone())));

    let light = Arc::new(diffuse_light::new_from_color(Vec3::new(50.0, 50.0, 50.0)));
    world.add(Arc::new(Sphere::new(Vec3::new(-20.0, 50.0, 10.0), 10.0, light.clone())));
    world.add(Arc::new(Sphere::new(Vec3::new(2.0, 50.0, -10.0), 10.0, light.clone())));


    let mut cam: Camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100 as u8, SAMPLES_PER_PIXEL, MAX_DEPTH, 
        VFOV, LOOKFROM, LOOKAT, VUP,
        DEFOCUS_ANGLE, FOCUS_DIST,
        BACKGROUND);

    let world = &mut (hittable_list::new_from_object(Arc::new(BVHNode::new_from_list(world))));
    cam.render(world);
}

fn main() {
    // bouncing_spheres();
    // checkered_spheres();
    // earth();
    // baihua();
    // perlin_spheres();
    // quads();
    // simple_light();
    cornell_box();
    // cornell_smoke();
    // final_scene(800, 5000, 40);
    // final_scene_mod(800, 5000, 40);
    // final_scene(200, 50, 40)
    // minecraft();
}
