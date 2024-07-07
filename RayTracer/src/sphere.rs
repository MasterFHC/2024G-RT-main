pub use crate::ray::Ray;
use crate::Vec3;
pub use crate::util::{fmax};
pub use crate::hittables::{hit_record, hittable};
use crate::Interval;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius: fmax(0.0, radius),
        }
    }
}

impl hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let oc = self.center - r.a_origin;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}