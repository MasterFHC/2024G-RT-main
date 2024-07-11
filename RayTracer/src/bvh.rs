use crate::ray::Ray;
use crate::intervals::Interval;
use crate::hittables::{hit_record, hittable, hittable_list};
use crate::aabb::AABB;
use std::rc::Rc;
use crate::util;

pub struct BVHNode {
    pub bbox: AABB,
    pub left: Rc<dyn hittable>,
    pub right: Rc<dyn hittable>,
}

impl BVHNode {
    pub fn new_from_list(list: &mut hittable_list) -> Self {
        let size = list.objects.len();
        Self::new(&mut list.objects, 0, size)
    }
    pub fn new(objects: &mut Vec<Rc<dyn hittable>>, start: usize, end: usize) -> Self {
        // println!("start: {}, end: {}", start, end);
        let axis = util::random_range_int(0, 3);

        let object_span = end - start;

        let left: Rc<dyn hittable>;
        let right: Rc<dyn hittable>;

        if object_span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
        } else if object_span == 2 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start + 1]);
        } else {
            objects[start..end].sort_by(|a, b| {a.bbox().axis_interval(axis as u8).tmin.partial_cmp(
                                                    &b.bbox().axis_interval(axis as u8).tmin
                                                ).unwrap()});
            let mid = start + object_span / 2;
            left = Rc::new(BVHNode::new(objects, start, mid));
            right = Rc::new(BVHNode::new(objects, mid, end));
        }
        Self {
            bbox: AABB::new_from_boxes(&left.bbox(), &right.bbox()),
            left,
            right,
        }
    }

    //define those box compare functions
    fn box_compare(a: &Rc<dyn hittable>, b: &Rc<dyn hittable>, axis: u8) -> bool {
        let box_a = a.bbox();
        let box_b = b.bbox();
        box_a.axis_interval(axis).tmin < box_b.axis_interval(axis).tmin
    }
    fn box_x_compare(a: &Rc<dyn hittable>, b: &Rc<dyn hittable>) -> bool {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Rc<dyn hittable>, b: &Rc<dyn hittable>) -> bool {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Rc<dyn hittable>, b: &Rc<dyn hittable>) -> bool {
        Self::box_compare(a, b, 2)
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