use crate::ray::Ray;
use crate::intervals::Interval;
use crate::hittables::{hit_record, hittable, hittable_list};
use crate::aabb::AABB;

pub struct BVHNode {
    pub bbox: AABB,
    pub left: Box<dyn hittable>,
    pub right: Box<dyn hittable>,
}

impl BVHNode {
    pub fn new(objects: &Vec<Box<dyn hittable>>, start: usize, end: usize) -> Self {
        let axis = util::random_range(0, 3);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Invalid axis"),
        };

        let object_span = end - start;

        
    }
}

impl hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut hit_record) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let mut new_ray_t = if hit_left {
            Interval::new(ray_t.tmin, rec.t)
        } else {
            Interval::new(ray_t.tmin, ray_t.tmax)
        };
        let hit_right = self.right.hit(r, &mut new_ray_t, rec);

        hit_left || hit_right
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}