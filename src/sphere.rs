use crate::vec3::{Point3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;

use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere{ center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // a vector dot product with itself == vector.length_squared()
        let a = ray.direction().length_squared();
        let b = 2.0 * (ray.origin() - self.center).dot(&ray.direction());
        let c = (ray.origin() - self.center).length_squared()
                - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 { return None; }

        let root1 = (-b - discriminant.sqrt()) / (2.0*a);
        let root2 = (-b + discriminant.sqrt()) / (2.0*a);
        let mut root = root1;

        
        if !ray_t.surrounds(root1) {
            if !ray_t.surrounds(root2) { return None; }
            else { root = root2; }
        } 
        
        // define outside normal of sphere to detect
        // the direction of ray (from outside or inside of sphere)
        // NOTE: you can use vector.unit_vector() to get unit normal,
        // but divide by radius is a trick for generating hollow glass sphere.
        let intersection = ray.at(root);
        let outside_normal = (intersection - self.center) / self.radius;
        if outside_normal.dot(&ray.direction()) <= 0.0 {
            Some(HitRecord{ p: intersection, 
                            normal: outside_normal, 
                            t: root, 
                            from_outside: true,
                            material: self.material.clone() })
        } else {
            Some(HitRecord{ p: intersection, 
                            normal: -outside_normal, 
                            t: root, 
                            from_outside: false,
                            material: self.material.clone() })
        }
    }
}
