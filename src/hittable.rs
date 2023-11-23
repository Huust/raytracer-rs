use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

use std::rc::Rc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>; 
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub from_outside: bool,  // record if ray come from outside object
    pub material: Rc<dyn Material>
}
