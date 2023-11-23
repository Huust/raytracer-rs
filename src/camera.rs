// camera, you need to:
// define camera center, focal length, specification of 
// viewport and image
// you need to initialize it, render the color then output it to stdout
use crate::vec3::{Vec3, Point3, Color};
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::interval::Interval;

use std::rc::Rc;
use std::cell::RefCell; 
use rand::prelude::*;
use std::f64::consts::PI;

const MAX_COLOR: u32 = 255;
const NUMS_SAMPLE: usize = 100;
const TIMES_REFLECTION: usize = 50; // maximum reflection times, otherwise may cause stackoverflow

pub struct Camera {
    aspect_ratio : f64,
    image_w      : u32,
    viewport_h   : f64,

    theta        : f64, // rust has const radian value, like FRAC_PI_4 == pi/4
    
    lookfrom     : Point3,
    lookat       : Point3,
    vup          : Vec3,
    focal_length : f64,
    defocus_angle : f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, theta: f64, defocus_angle: f64, focus_dist: f64) -> Camera {
        Camera{ aspect_ratio: 1.0,
                image_w     : 400,
                viewport_h  : 3.0,
                lookfrom, lookat, theta,
                vup: Vec3::new(0.0, 1.0, 0.0),
                focal_length: (lookfrom - lookat).length(),
                defocus_angle, focus_dist,
                defocus_disk_v:Vec3::new(0.0, 0.0, 0.0), 
                defocus_disk_u:Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn initialize(&mut self, aspect_ratio: f64, image_w: u32) {
        self.aspect_ratio = aspect_ratio;
        self.image_w = image_w;
        // update viewport_h with theta
        self.viewport_h = 2.0 * self.focal_length * (self.theta / 2.0).tan();
    }
    
    pub fn render(&mut self, world: Rc<RefCell<dyn Hittable>>) {
        

        let camera_center = self.lookfrom;
        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = ((-w).cross(&self.vup.unit_vector())).unit_vector();
        let v = u.cross(&-w).unit_vector();

        let defocus_radius = self.focus_dist * (2.0*PI*(self.defocus_angle/2.0)/360.0).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;

        self.viewport_h = 2.0 * self.focus_dist * (self.theta/2.0).tan();
       
        // related variables initialization
        let image_h = (self.image_w as f64 / self.aspect_ratio) as u32;
        println!("P3\n{} {}\n{}", self.image_w, image_h, MAX_COLOR);

        let viewport_w = self.viewport_h * (self.image_w as f64 / image_h as f64);
        // viewport_u is direction vector, its length is viewport_w, its direction 
        // specifies the direction of longer edge(width)
        // viewport_v is same as viewport_u, but specifies the direction of height
        let viewport_u = viewport_w * u; 
        let viewport_v = self.viewport_h * v;

        // define the upper left position of viewport
        // camera_center - (focal_length * w) approximates position of lookat
        let vp_upper_left_pos: Point3 = 
        camera_center - (self.focus_dist * w) - viewport_u/2.0 + viewport_v/2.0;

        // define the gap between two pixels, then define the position of upper left pixel
        let delta_u = viewport_u / self.image_w as f64;
        let delta_v = viewport_v / image_h as f64;
        let pixel00_loc: Point3 = vp_upper_left_pos + 0.5 * (delta_u - delta_v);

        // rendering...
        for i in 0..image_h {
            eprintln!("Scanning line {}/{}...", i, image_h - 1);
            for j in 0..self.image_w {
                let rays = self.get_sample_ray(i, j, pixel00_loc, delta_u, delta_v);
                Self::write_color(rays, Rc::clone(&world));


                if j == self.image_w - 1 {
                    print!("\n");
                } else {
                    print!("  ");
                }
            }
        }
        eprintln!("Scanning done!");
    }

    // determine 10 random pixels in current square, get their rays
    fn get_sample_ray(&self, i: u32, j: u32, pixel00_loc: Point3, delta_u: Vec3, delta_v: Vec3) -> Vec<Ray> {
        let mut rays = vec![];
        let center_pixel: Point3 = pixel00_loc + j as f64 * delta_u
                                        - i as f64 * delta_v;
        let mut rng = thread_rng();

        // generate 10 random pixels coordinate
        // random number randoms from [-0.5, +0.5]
        for _ in 0..NUMS_SAMPLE {
            let random = rng.gen_range(-0.5..=0.5);
            let random_pixel = center_pixel + random * delta_u + random * delta_v;
            let origin = if self.defocus_angle <= 0.0 {self.lookfrom} else {self.defocus_disk_sample()};
            // rays.push(Ray::new(self.lookfrom, random_pixel - self.lookfrom));
            rays.push(Ray::new(origin, random_pixel - origin));
        }

        rays
    }

    // Given a ray at some position in world, what is its color?
    fn ray_color(ray: &Ray, world: Rc<RefCell<dyn Hittable>>, depth: usize) -> Color {
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5*(unit_direction.y() + 1.0);
        let default_color = (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);

        if depth <= 0 { return default_color; }

        match world.borrow().hit(ray, Interval::new(0.001, std::f64::INFINITY)) {
            None => default_color,
            Some(record) => {
                match record.material.scatter(ray, &record) {
                    None => Color::new(0.0, 0.0, 0.0),
                    Some((attenuation, scattered_ray)) => {
                        attenuation * Self::ray_color(&scattered_ray, Rc::clone(&world), depth - 1)
                    }
                }
            }
        }
    }

    // Write color(RGB) to stdout.
    fn write_color(rays: Vec<Ray>, world: Rc<RefCell<dyn Hittable>>) {
        let mut count: usize = 0;
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

        for ray in &rays {
            count += 1;
            pixel_color += Self::ray_color(ray, Rc::clone(&world), TIMES_REFLECTION);
        }
        pixel_color /= count as f64;
        pixel_color.sqrt(); // linear to gamma transform

        print!("{} {} {}", (MAX_COLOR as f64 * pixel_color.x()) as u32,
                           (MAX_COLOR as f64 * pixel_color.y()) as u32,
                           (MAX_COLOR as f64 * pixel_color.z()) as u32);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        
        self.lookfrom + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
