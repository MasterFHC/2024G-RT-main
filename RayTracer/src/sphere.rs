pub use crate::ray::Ray;
use crate::Vec3;
pub use crate::util::{fmax};
pub use crate::hittables::{hit_record, hittable};
use crate::materials::{material};
use crate::Interval;
use std::sync::Arc;
use crate::aabb::AABB;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,

    //material
    pub mat: Arc<dyn material + Send + Sync>,

    //moving parts
    is_moving: bool,
    center_vec: Vec3,

    //bounding box
    pub bbox: AABB,
}

impl Sphere {
    // Stationary sphere
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn material + Send + Sync>) -> Self {
        let rvec = Vec3::new(fmax(0.0, radius), fmax(0.0, radius), fmax(0.0, radius));
        Self {
            center,
            radius: fmax(0.0, radius),
            mat,

            //moving parts
            is_moving: false,
            center_vec: Vec3::zero(),

            //bounding box
            bbox: AABB::new_from_points(center - rvec, center + rvec),
        }
    }
    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn material + Send + Sync>) -> Self {
        let rvec = Vec3::new(fmax(0.0, radius), fmax(0.0, radius), fmax(0.0, radius));
        let box1 = AABB::new_from_points(center1 - rvec, center1 + rvec);
        let box2 = AABB::new_from_points(center2 - rvec, center2 + rvec);
        Self {
            center: center1,
            radius: fmax(0.0, radius),
            mat,

            //moving parts
            is_moving: true,
            center_vec: center2 - center1,

            //bounding box
            bbox: AABB::new_from_boxes(&box1, &box2),
        }
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        self.center + self.center_vec * time
    }
    fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
}

impl hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        let center = if self.is_moving { self.sphere_center(r.time) } else { self.center };
        let oc = center - r.a_origin;
        let a = r.b_direction.squared_length();
        let h = oc * r.b_direction;
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let mut root = (h - discriminant.sqrt()) / a;
        if (!ray_t.surrounds(root)) {
            root = (h + discriminant.sqrt()) / a;
            if (!ray_t.surrounds(root)) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Self::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        //dyn type can not be cloned, we use refrence count Arc to clone it
        rec.mat = Arc::clone(&self.mat);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}