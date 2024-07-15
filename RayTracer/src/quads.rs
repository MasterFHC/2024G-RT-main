pub use crate::ray::Ray;
use crate::Vec3;
use crate::util;
pub use crate::hittables::{hit_record, hittable};
use crate::materials::{material};
use crate::Interval;
use std::sync::Arc;
use crate::aabb::AABB;

pub struct quad {
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn material + Send + Sync>,

    //bounding box
    bbox: AABB,

    //temp variables
    normal: Vec3,
    D: f64,
}

impl quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn material + Send + Sync>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let D = normal * Q;
        let w = n * (1.0 / (n * n));
        let new_bbox = Self::set_bbox(Q, u, v);
        // println!("quad bbox: [{},{}] [{},{}] [{},{}]", new_bbox.x.tmin as f64, new_bbox.x.tmax as f64, 
        //                                                new_bbox.y.tmin as f64, new_bbox.y.tmax as f64, 
        //                                                new_bbox.z.tmin as f64, new_bbox.z.tmax as f64);
        Self {
            Q,
            u,
            v,
            w,
            mat,

            //bounding box
            bbox: new_bbox,

            //temp variables
            normal,
            D,
        }
    }
    fn set_bbox(Q: Vec3, u: Vec3, v: Vec3) -> AABB {
        let bbox_diag1 = AABB::new_from_points(Q, Q + u + v);
        let bbox_diag2 = AABB::new_from_points(Q + u, Q + v);
        AABB::new_from_boxes(&bbox_diag1, &bbox_diag2)
    }
    fn is_interior(alpha: f64, beta: f64, rec: &mut hit_record) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }
        rec.u = alpha;
        rec.v = beta;
        // println!("got u: {}, got v: {}", rec.u, rec.v);
        true
    }
}

impl hittable for quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        let denom = self.normal * r.b_direction;

        //No hit if ray is parallel to the plane
        if util::fabs(denom) < 1e-8 {
            return false;
        }

        //Return false if the hit point t is outside the ray_t interval
        let t = (self.D - self.normal * r.a_origin) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        //Determine the hit point lies within the quad using its plane coordinates

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w * (planar_hitpt_vector.cross(self.v));
        let beta = self.w * (self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }
        // println!("actually got u: {}, got v: {}", rec.u, rec.v);

        //Ray hits the 2D shape; set hit record
        rec.t = t;
        rec.p = intersection;
        rec.mat = Arc::clone(&self.mat);
        rec.set_face_normal(r, &self.normal);

        true
    }
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}