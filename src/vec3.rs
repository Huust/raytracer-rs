use std::ops::{Neg, AddAssign, MulAssign, DivAssign, Index, IndexMut};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use rand::prelude::*;

// Actually tuple struct is more recommended under this circumstance
// Vec3(f64, f64, f64)
#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{ e: [x, y, z] } 
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x()
      + self.y() * other.y()
      + self.z() * other.z()
    }
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(self.e[1] * other.e[2] - self.e[2] * other.e[1],
                  self.e[2] * other.e[0] - self.e[0] * other.e[2],
                  self.e[0] * other.e[1] - self.e[1] * other.e[0])
    }
    pub fn unit_vector(&self) -> Self {
        *self / self.length() 
    }
    pub fn sqrt(&mut self) {
        for i in 0..3 {
            self.e[i] = self.e[i].sqrt();
        }
    }
    
    // generate a random vector
    pub fn random(min: f64, max: f64) -> Vec3 {
        let rng = thread_rng;
        Vec3::new(rng().gen_range(min..max),
                  rng().gen_range(min..max), 
                  rng().gen_range(min..max))
    }
    // generate a unit vector
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            match p.length_squared() > 1.0 {
                true => { continue; }
                false => { break p.unit_vector(); }
            }
        }
    }
    // check whether three directions of a vector all near zero
    // this is used to determine whether sum of two vectors 
    // comes from two reverse vectors
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
    // use input ray, intersection point's normal and refraction index to 
    // determine output ray from dielectrics
    // ratio is input environment refractive index / output environment refractive index
    pub fn refract(unit_input: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
        let vertical = ratio * (unit_input + normal.dot(&-unit_input) * normal);
        let parallel = -normal * (1.0 - vertical.length_squared()).sqrt();
        
        vertical + parallel
    }
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut p = Vec3::random(-1.0, 1.0);
            p.e[2] = 0.0; 

            match p.length_squared() < 1.0 {
                true  => {break p;}
                false => {continue;}
            }
        }
    }
}

// overload operators for Vec3
// BTW, you can use keywords "std::ops" for search of overloadable operators in rust
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 3 { panic!("Index out of Vec3 bound!") }
        
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 3 { panic!("Index out of Vec3 bound!") }
        
        &mut self.e[index]
    }
}
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}


impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x()+other.x(), self.y()+other.y(), self.z()+other.z())
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x()-other.x(), self.y()-other.y(), self.z()-other.z())
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x()*other.x(), self.y()*other.y(), self.z()*other.z())
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(self.x()*other, self.y()*other, self.z()*other)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        if other == 0.0 {
            panic!("Cannot divide by zero-valued `f64`");
        }
        // Vec3::new(self.x()/other, self.y()/other, self.z()/other) 
        self * (1.0/other)
    }
}


// implement Display trait for Vec3
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// type alias
pub type Point3 = Vec3;
pub type Color = Vec3;
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.x(); 
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}
