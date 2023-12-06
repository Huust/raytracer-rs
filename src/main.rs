mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;
use vec3::{Point3, Color};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use material::{Material, Lambertian as Lamber, Metal, Dielectrics as Die};

use std::sync::Arc;
use rand::prelude::*;
// use std::f64::consts::FRAC_PI_2;
// use std::f64::consts::FRAC_PI_4;
use std::f64::consts::FRAC_PI_8;

const IMAGE_WIDTH: u32  = 1200;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lamber::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material))); 

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen(); // random number in [0, 1)
            let center = Point3::new(a as f64 + 0.9*rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9*rng.gen_range(0.0..1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    sphere_material = Arc::new(Lamber::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Die::new(Color::new(1.0, 1.0, 1.0), 1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Die::new(Color::new(1.0, 1.0, 1.0), 1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    let material2 = Arc::new(Lamber::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let world_ref = Arc::new(world);

    let mut camera = Camera::new(Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), FRAC_PI_8, 0.6, 10.0);
    camera.initialize(ASPECT_RATIO, IMAGE_WIDTH);
    camera.render(world_ref);
}
