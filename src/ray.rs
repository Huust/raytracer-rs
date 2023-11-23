use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir : Vec3
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray{ orig, dir } 
    }
    
    pub fn origin(&self) -> Point3 {
        Point3::new(self.orig.x(), self.orig.y(), self.orig.z()) 
    }
    pub fn direction(&self) -> Vec3 {
        Vec3::new(self.dir.x(), self.dir.y(), self.dir.z()) 
    }

    // pass in an argument t can determine a point on this line, starting
    // from the origin
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir 
    }
}
