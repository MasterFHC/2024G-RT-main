pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::Interval;
use crate::materials::{material, lambertian, metal};
use std::rc::Rc;
use crate::aabb::AABB;

pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,

    //material
    pub mat: Rc<dyn material>,
}

impl hit_record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        //we assume here that outward_normal is a unit vector
        self.front_face = (r.b_direction * *outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * (-1.0);
        }
    }
}

pub trait hittable {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut hit_record) -> bool;
    fn bbox(&self) -> &AABB;
}

pub struct hittable_list {
    pub objects: Vec<Box<dyn hittable>>,
    pub bbox: AABB,
}

impl hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new(Interval::new(f64::INFINITY, f64::INFINITY), 
                            Interval::new(f64::INFINITY, f64::INFINITY), 
                            Interval::new(f64::INFINITY, f64::INFINITY)),
        }
    }
    pub fn add(&mut self, object: Box<dyn hittable>) {
        self.bbox = AABB::new_from_boxes(&self.bbox, &object.bbox());
        self.objects.push(object);
    }
}

impl hittable for hittable_list {

    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut hit_record) -> bool {
        let mut rec_temp = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(lambertian { albedo: Vec3::zero() }),
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.tmax;

        for object in self.objects.iter() {
            if object.hit(r, &mut Interval::new(ray_t.tmin, closest_so_far), &mut rec_temp) {
                hit_anything = true;
                closest_so_far = rec_temp.t;

                //manually copy rec_temp to rec
                rec.p = rec_temp.p;
                rec.normal = rec_temp.normal;
                rec.t = rec_temp.t;
                rec.front_face = rec_temp.front_face;
                rec.mat = Rc::clone(&rec_temp.mat);
            }
        }

        hit_anything
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}