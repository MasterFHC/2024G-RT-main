use crate::intervals::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use core::ops::Add;

pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut xx = x;
        let mut yy = y;
        let mut zz = z;
        Self::pad_to_minimums(xx, yy, zz)
    }
    pub fn new_from_points(p0: Vec3, p1: Vec3) -> Self {
        let x = if p0.x < p1.x {
            Interval::new(p0.x, p1.x)
        } else {
            Interval::new(p1.x, p0.x)
        };
        let y = if p0.y < p1.y {
            Interval::new(p0.y, p1.y)
        } else {
            Interval::new(p1.y, p0.y)
        };
        let z = if p0.z < p1.z {
            Interval::new(p0.z, p1.z)
        } else {
            Interval::new(p1.z, p0.z)
        };
        let mut xx = x;
        let mut yy = y;
        let mut zz = z;
        Self::pad_to_minimums(xx, yy, zz)
    }
    pub fn new_from_boxes(box0: &AABB, box1: &AABB) -> Self {
        let mut xx = Interval::new_from_interval(&box0.x, &box1.x);
        let mut yy = Interval::new_from_interval(&box0.y, &box1.y);
        let mut zz = Interval::new_from_interval(&box0.z, &box1.z);
        Self::pad_to_minimums(xx, yy, zz)
    }
    pub fn axis_interval(&self, n: u8) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }
    pub fn longest_axis(&self) -> u8 {
        let mut axis = 0;
        let mut max_size = self.x.size();
        if self.y.size() > max_size {
            axis = 1;
            max_size = self.y.size();
        }
        if self.z.size() > max_size {
            axis = 2;
        }
        axis
    }
    fn pad_to_minimums(x: Interval, y: Interval, z: Interval) -> Self {
        let delta = 0.0001;
        let x = if x.size() < delta { x.expand(delta) } else { x };
        let y = if y.size() < delta { y.expand(delta) } else { y };
        let z = if z.size() < delta { z.expand(delta) } else { z };
        Self {x, y, z}
    }

    pub fn hit(&self, r: &Ray, org_ray_t: &Interval) -> bool{
        let ray_origin = r.a_origin;
        let ray_direction = r.b_direction;
        let mut ray_t = Interval::new(org_ray_t.tmin, org_ray_t.tmax);

        for i in 0..3 {
            let adinv = 1.0 / ray_direction.lp(i);
            let ax = self.axis_interval(i);

            let mut t0 = (ax.tmin - ray_origin.lp(i)) * adinv;
            let mut t1 = (ax.tmax - ray_origin.lp(i)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.tmin {
                    ray_t.tmin = t0;
                }
                if t1 < ray_t.tmax {
                    ray_t.tmax = t1;
                }
            }
            else {
                if t1 > ray_t.tmin {
                    ray_t.tmin = t1;
                }
                if t0 < ray_t.tmax {
                    ray_t.tmax = t0;
                }
            }

            if ray_t.tmax <= ray_t.tmin {
                return false;
            }
        }
        true
    }
}

impl Add<Vec3> for &AABB {
    type Output = AABB;

    fn add(self, other: Vec3) -> AABB {
        let x = &self.x + other.x;
        let y = &self.y + other.y;
        let z = &self.z + other.z;
        AABB {
            x,
            y,
            z,
        }
    }
}