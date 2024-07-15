use crate::util;
use core::ops::Add;

pub struct Interval {
    pub tmin: f64,
    pub tmax: f64,
}

impl Interval {
    pub fn new(tmin: f64, tmax: f64) -> Self {
        Self {
            tmin: util::fmin(tmin, tmax),
            tmax: util::fmax(tmin, tmax),
        }
    }
    pub fn new_from_interval(a: &Interval, b: &Interval) -> Self {
        Self {
            tmin: util::fmin(a.tmin, b.tmin),
            tmax: util::fmax(a.tmax, b.tmax),
        }
    }
    pub fn size(&self) -> f64 {
        self.tmax - self.tmin
    }
    pub fn contains(&self, t: f64) -> bool {
        t >= self.tmin && t <= self.tmax
    }
    pub fn surrounds(&self, t: f64) -> bool {
        t > self.tmin && t < self.tmax
    }
    pub fn clamp(&self, t: f64) -> f64 {
        if t < self.tmin {
            self.tmin
        } else if t > self.tmax {
            self.tmax
        } else {
            t
        }
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta * 0.5;
        Self {
            tmin: self.tmin - padding,
            tmax: self.tmax + padding,
        }
    }
}

impl Add<f64> for &Interval {
    type Output = Interval;

    fn add(self, other: f64) -> Interval {
        Interval {
            tmin: self.tmin + other,
            tmax: self.tmax + other,
        }
    }
}

const empty: Interval = Interval { tmin: f64::INFINITY, tmax: f64::NEG_INFINITY };
const universe: Interval = Interval { tmin: f64::NEG_INFINITY, tmax: f64::INFINITY };