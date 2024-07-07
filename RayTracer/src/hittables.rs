pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::Interval;

pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
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
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool;
}

pub struct hittable_list {
    pub objects: Vec<Box<dyn hittable>>,
}

impl hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Box<dyn hittable>) {
        self.objects.push(object);
    }
}

impl hittable for hittable_list {

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let mut rec_temp = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.tmax;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.tmin, closest_so_far), &mut rec_temp) {
                hit_anything = true;
                closest_so_far = rec_temp.t;

                //manually copy rec_temp to rec
                rec.p = rec_temp.p;
                rec.normal = rec_temp.normal;
                rec.t = rec_temp.t;
                rec.front_face = rec_temp.front_face;
            }
        }

        hit_anything
    }
}