use crate::vec3::{Vec3, Color};
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Material {
    // this function's responsibility:
    // determine whether this material reflect rays
    // if so, give out the reflected ray and attenuation
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Color    
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{ albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let direction = Vec3::random_in_unit_sphere() + record.normal; 
        Some((self.albedo, Ray::new(record.p, 
            if direction.near_zero() { record.normal } else { direction }))) 
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz:   f64
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal{ albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let direction = ray_in.direction() - 2.0 * record.normal.dot(&ray_in.direction()) * record.normal;
        //let direction = 2.0 * record.normal + ray_in.direction();
        let r = Ray::new(record.p, direction + self.fuzz*Vec3::random_in_unit_sphere());
        if r.direction().dot(&record.normal) > 0.0 {
            Some((self.albedo, r))
        } else { None }
    }
}

pub struct Dielectrics {
    albedo: Color,
    index:  f64,    // refraction index
}
impl Dielectrics {
    pub fn new(albedo: Color, index: f64) -> Self {
        Dielectrics{ albedo, index }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        return r0 + (1.0-r0)*(1.0 - cosine).powi(5);
    }
}
impl Material for Dielectrics {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let unit_direction = ray_in.direction().unit_vector();
        let from_outside = record.from_outside;
        let refraction_ratio = if from_outside { 1.0 / self.index } else { self.index };

        let cos_theta = -unit_direction.dot(&record.normal);
        let refracted_direction  = Vec3::refract(unit_direction,
        record.normal, refraction_ratio);
        
        // schlick approximation
        if Self::reflectance(cos_theta, refraction_ratio) > 0.6 {
            let metal = Metal::new(self.albedo, 0.0);
            return metal.scatter(ray_in, record);
        }
       
        // we use snell's law to calculate refracted ray,
        // but sometimes this law doesn't work
        match from_outside {
            true => {
                Some((self.albedo, Ray::new(record.p, refracted_direction)))         
            }
            false => {
                let is_reflection = refraction_ratio * (1.0 - cos_theta.powi(2)).sqrt() > 1.0;
                // if ray from inside, check whether snell's law still work
                if !is_reflection {
                    Some((self.albedo, Ray::new(record.p, refracted_direction)))
                } else {
                    // otherwise, it's reflection not refraction
                    // we regard it as metal reflection
                    let reflected_direction = unit_direction + 2.0*cos_theta*record.normal;
                    Some((self.albedo, Ray::new(record.p, reflected_direction)))
                }
            }
        }
    }
}
