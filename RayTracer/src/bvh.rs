use crate::ray::Ray;
use crate::intervals::Interval;
use crate::hittables::{hit_record, hittable, hittable_list};
use crate::aabb::AABB;
use std::sync::Arc;
use crate::util;

pub struct BVHNode {
    pub bbox: AABB,
    pub left: Arc<dyn hittable + Send + Sync>,
    pub right: Arc<dyn hittable + Send + Sync>,
}

impl BVHNode {
    pub fn new_from_list(list: &mut hittable_list) -> Self {
        let size = list.objects.len();
        Self::new(&mut list.objects, 0, size)
    }
    pub fn new(objects: &mut Vec<Arc<dyn hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        // println!("start: {}, end: {}", start, end);
        let axis = util::random_range_int(0, 3);

        let object_span = end - start;

        let left: Arc<dyn hittable + Send + Sync>;
        let right: Arc<dyn hittable + Send + Sync>;

        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start + 1]);
        } else {
            objects[start..end].sort_by(|a, b| {a.bbox().axis_interval(axis as u8).tmin.partial_cmp(
                                                    &b.bbox().axis_interval(axis as u8).tmin
                                                ).unwrap()});
            let mid = start + object_span / 2;
            left = Arc::new(BVHNode::new(objects, start, mid));
            right = Arc::new(BVHNode::new(objects, mid, end));
        }
        Self {
            bbox: AABB::new_from_boxes(&left.bbox(), &right.bbox()),
            left,
            right,
        }
    }

    //define those box compare functions
    fn box_compare(a: &Arc<dyn hittable + Send + Sync>, b: &Arc<dyn hittable + Send + Sync>, axis: u8) -> bool {
        let box_a = a.bbox();
        let box_b = b.bbox();
        box_a.axis_interval(axis).tmin < box_b.axis_interval(axis).tmin
    }
}

impl hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut hit_record) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let new_ray_t = if hit_left {
            Interval::new(ray_t.tmin, rec.t)
        } else {
            Interval::new(ray_t.tmin, ray_t.tmax)
        };
        let hit_right = self.right.hit(r, &new_ray_t, rec);

        hit_left || hit_right
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}