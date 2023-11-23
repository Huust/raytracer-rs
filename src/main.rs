mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod utility;
mod interval;
mod camera;
mod material;
use vec3::{Point3, Color};
use sphere::Sphere;
use hittable::Hittable;
use hittable_list::HittableList;
use camera::Camera;
use material::{Material, Lambertian as Lamber, Metal, Dielectrics as Die};

use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;
// use std::f64::consts::FRAC_PI_2;
// use std::f64::consts::FRAC_PI_4;
use std::f64::consts::FRAC_PI_8;


fn main() {
    let  world = Rc::new(RefCell::new(HittableList::new()));

    let ground_material = Rc::new(Lamber::new(Color::new(0.5, 0.5, 0.5)));
    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material)))); 

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen(); // random number in [0, 1)
            let center = Point3::new(a as f64 + 0.9*rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9*rng.gen_range(0.0..1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    sphere_material = Rc::new(Lamber::new(albedo));
                    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_material))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_material))));
                } else {
                    // glass
                    sphere_material = Rc::new(Die::new(Color::new(1.0, 1.0, 1.0), 1.5));
                    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_material))));
                }
            }
        }
    }

    let material1 = Rc::new(Die::new(Color::new(1.0, 1.0, 1.0), 1.5));
    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1))));
    let material2 = Rc::new(Lamber::new(Color::new(0.4, 0.2, 0.1)));
    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2))));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.borrow_mut().add(Rc::new(RefCell::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3))));

    let mut camera = Camera::new(Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), FRAC_PI_8, 0.6, 10.0);
    camera.initialize(16.0/9.0, 1200);
    camera.render(Rc::clone(&world) as Rc<RefCell<dyn Hittable>>);
}