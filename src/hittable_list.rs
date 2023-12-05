// We have a generic object called a hittable that the ray can intersect with.
// We now add a class that stores a list of hittables.
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Metal;

use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{ objects: vec![] } 
    }
    
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object); 
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut interval = ray_t;
        let mut rec = HitRecord{ p: Point3::new(0.0, 0.0, 0.0),
                                 normal: Vec3::new(0.0, 0.0, 0.0),
                                 t: 0.0,
                                 from_outside: false, 
                                 material: Arc::new(Metal::new(Color::new(0.0, 0.0, 0.0), 1.0)) };
                                 
        
        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, interval) {
                hit_anything = true;
                interval.set_max(record.t);
                rec = record;
            }
        }

        match hit_anything {
            true  => Some(rec),
            false => None
        }
    }
}
