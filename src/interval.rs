// This file defines a class of interval, 
// which contains two values: min and max,
// we can detect whether a f64 value lay between min and max

#[derive(Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval{ min, max } 
    }
    
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn set_max(&mut self, x: f64) {
        self.max = x;
    }
}
