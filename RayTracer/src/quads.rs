pub use crate::ray::Ray;
use crate::Vec3;
use crate::util;
pub use crate::hittables::{hit_record, hittable, hittable_list};
use crate::materials::{material, lambertian};
use crate::textures::{Image};
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
        // println!("quad bbox: [{}, {}], [{}, {}], [{}, {}]", new_bbox.x.tmin, new_bbox.x.tmax, new_bbox.y.tmin, new_bbox.y.tmax, new_bbox.z.tmin, new_bbox.z.tmax);
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

pub fn newbox(a: Vec3, b: Vec3, mat: Arc<dyn material + Send + Sync>) -> hittable_list {
    // println!("newbox");
    //Returns the 3D box (six sides) that contains the two opposite vertices a & b
    let mut sides = hittable_list::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Vec3::new(util::fmin(a.x, b.x), util::fmin(a.y, b.y), util::fmin(a.z, b.z));
    let max = Vec3::new(util::fmax(a.x, b.x), util::fmax(a.y, b.y), util::fmax(a.z, b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    let wsh_texture = Arc::new(Image::new("wsh_light.png"));
    let wsh_surface = Arc::new(lambertian::new_from_texture(wsh_texture.clone()));

    sides.add(Arc::new(quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, mat.clone())));//front
    sides.add(Arc::new(quad::new(Vec3::new(max.x, min.y, max.z), dz * (-1.0), dy, mat.clone())));//right
    sides.add(Arc::new(quad::new(Vec3::new(max.x, min.y, min.z), dx * (-1.0), dy, mat.clone())));//back
    sides.add(Arc::new(quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, mat.clone())));//left
    // sides.add(Arc::new(quad::new(Vec3::new(min.x, max.y, max.z), dx, dz * (-1.0), mat.clone())));//top
    sides.add(Arc::new(quad::new(Vec3::new(min.x, max.y, max.z), dx, dz * (-1.0), wsh_surface.clone())));//top with wsh texture
    sides.add(Arc::new(quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, mat.clone())));//bottom

    // println!("bbox of box: [{}, {}], [{}, {}], [{}, {}]", sides.bbox.x.tmin, sides.bbox.x.tmax, sides.bbox.y.tmin, sides.bbox.y.tmax, sides.bbox.z.tmin, sides.bbox.z.tmax);

    sides
}