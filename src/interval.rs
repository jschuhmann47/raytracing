use core::f64;

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Returns an interval from ``f64::MIN`` to ``f64::MAX``
    pub fn default() -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
        }
    }

    pub fn contains_inclusive(&self, num: f64) -> bool {
        num >= self.min && num <= self.max
    }

    pub fn contains(&self, num: f64) -> bool {
        num > self.min && num < self.max
    }
}
