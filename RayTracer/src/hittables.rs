pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::Interval;
use crate::materials::{material, lambertian, isotropic};
use std::sync::Arc;
use crate::aabb::AABB;
use crate::SolidColor;
use crate::util;
use crate::textures::texture;

pub struct hit_record{
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,

    //material
    pub mat: Arc<dyn material + Send + Sync>,

    //texture
    pub u: f64,
    pub v: f64,
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

pub trait hittable : Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool;
    fn bbox(&self) -> &AABB;
}

pub struct hittable_list{
    pub objects: Vec<Arc<dyn hittable + Send + Sync>>,
    pub bbox: AABB,

    is_first_bbox: bool,
}

impl hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new(Interval::new(0.0, 0.0), 
                            Interval::new(0.0, 0.0), 
                            Interval::new(0.0, 0.0)),
            is_first_bbox: true,
        }
    }
    pub fn new_from_object(object: Arc<dyn hittable + Send + Sync>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }
    pub fn add(&mut self, object: Arc<dyn hittable + Send + Sync>) {
        let obj_clone = object.clone();
        let cloned_bbox = obj_clone.bbox();
        self.objects.push(object);
        if self.is_first_bbox {
            self.bbox = AABB::new(Interval::new(cloned_bbox.x.tmin, cloned_bbox.x.tmax),
                                  Interval::new(cloned_bbox.y.tmin, cloned_bbox.y.tmax),
                                  Interval::new(cloned_bbox.z.tmin, cloned_bbox.z.tmax));
            self.is_first_bbox = false;
        } else {
            self.bbox = AABB::new_from_boxes(&self.bbox, obj_clone.bbox());
        }
    }
}

impl hittable for hittable_list {

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        let mut rec_temp = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            // mat: Arc::new(lambertian { tex: Arc::new(SolidColor::new(Vec3::zero())) }),
            mat: Arc::new(lambertian::new_from_texture(Arc::new(SolidColor::new(Vec3::zero())))),
            u: 0.0,
            v: 0.0,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.tmax;

        for object in self.objects.iter() {
            if object.hit(r, &Interval::new(ray_t.tmin, closest_so_far), &mut rec_temp) {
                hit_anything = true;
                closest_so_far = rec_temp.t;

                //manually copy rec_temp to rec
                rec.p = rec_temp.p;
                rec.normal = rec_temp.normal;
                rec.t = rec_temp.t;
                rec.front_face = rec_temp.front_face;
                rec.mat = Arc::clone(&rec_temp.mat);
                rec.u = rec_temp.u;
                rec.v = rec_temp.v;
            }
        }

        hit_anything
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct translate {
    pub object: Arc<dyn hittable + Send + Sync>,
    pub offset: Vec3,
    bbox: AABB,
}

impl translate {
    pub fn new(object: Arc<dyn hittable + Send + Sync>, offset: Vec3) -> Self {
        let other_object = object.clone();
        let newbbox = (other_object.bbox()) + offset;
        Self {
            object,
            offset,
            bbox: newbbox,
        }
    }
}

impl hittable for translate {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        let offset_r = Ray::new(r.a_origin - self.offset, r.b_direction, r.time);
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;
        // rec.set_face_normal(&moved_r, &rec.normal);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct rotate_y {
    pub object: Arc<dyn hittable + Send + Sync>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: AABB,
}

impl rotate_y {
    pub fn new(object: Arc<dyn hittable + Send + Sync>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bbox();

        let mut min = vec!(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = vec!(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.tmax + (1 - i) as f64 * bbox.x.tmin;
                    let y = j as f64 * bbox.y.tmax + (1 - j) as f64 * bbox.y.tmin;
                    let z = k as f64 * bbox.z.tmax + (1 - k) as f64 * bbox.z.tmin;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = vec!(newx, y, newz);

                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                    }
                }
            }
        }

        let min = Vec3::new(min[0], min[1], min[2]);
        let max = Vec3::new(max[0], max[1], max[2]);

        let newbbox = AABB::new_from_points(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: newbbox,
        }
    }
}

impl hittable for rotate_y {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        // Change the ray from world space to object space
        let mut origin = r.a_origin;
        let mut direction = r.b_direction;

        origin.x = self.cos_theta * r.a_origin.x - self.sin_theta * r.a_origin.z;
        origin.z = self.sin_theta * r.a_origin.x + self.cos_theta * r.a_origin.z;

        direction.x = self.cos_theta * r.b_direction.x - self.sin_theta * r.b_direction.z;
        direction.z = self.sin_theta * r.b_direction.x + self.cos_theta * r.b_direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where)
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        // Change the intersection point from object space to world space
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        // Change the normal from object space to world space
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct constant_medium {
    pub boundary: Arc<dyn hittable + Send + Sync>,
    pub neg_inv_density: f64,
    pub phase_function: Arc<dyn material + Send + Sync>,
}

impl constant_medium {
    pub fn new_from_texture(boundary: Arc<dyn hittable + Send + Sync>, density: f64, tex: Arc<dyn texture + Send + Sync>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(isotropic::new_from_texture(tex)),
        }
    }
    pub fn new(boundary: Arc<dyn hittable + Send + Sync>, density: f64, color: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(isotropic::new(color)),
        }
    }
}

impl hittable for constant_medium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        let mut rec1 = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(lambertian::new_from_texture(Arc::new(SolidColor::new(Vec3::zero())))),
            u: 0.0,
            v: 0.0,
        };
        let mut rec2 = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(lambertian::new_from_texture(Arc::new(SolidColor::new(Vec3::zero())))),
            u: 0.0,
            v: 0.0,
        };

        if !self.boundary.hit(r, &Interval::new(f64::NEG_INFINITY, f64::INFINITY), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, &Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2) {
            return false;
        }

        if rec1.t < ray_t.tmin {
            rec1.t = ray_t.tmin;
        }

        if rec2.t > ray_t.tmax {
            rec2.t = ray_t.tmax;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.b_direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * util::random_f64_0_1().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = Arc::clone(&self.phase_function);

        true
    }

    fn bbox(&self) -> &AABB {
        self.boundary.bbox()
    }
}