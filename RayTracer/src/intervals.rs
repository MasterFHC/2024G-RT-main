pub struct Interval {
    pub tmin: f64,
    pub tmax: f64,
}

impl Interval {
    pub fn new(tmin: f64, tmax: f64) -> Self {
        Self {
            tmin,
            tmax,
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
}

const empty: Interval = Interval { tmin: f64::INFINITY, tmax: f64::NEG_INFINITY };
const universe: Interval = Interval { tmin: f64::NEG_INFINITY, tmax: f64::INFINITY };