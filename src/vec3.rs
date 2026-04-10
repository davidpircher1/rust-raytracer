use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};
use rand::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Copy, Clone,PartialEq, Serialize, Deserialize)] 
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    // getters
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    // umocnena dlzka
    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    // skutocna dlzka
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    // jednotkovy vektor 
    pub fn unit_vector(self) -> Vec3 {
        self / self.length() // vyuzitie impl div 
    }

    // skalarny sucin
    pub fn dot(self, other: Vec3) -> f64{
        self.e[0] * other.e[0] +
        self.e[1] * other.e[1] +
        self.e[2] * other.e[2]
    }

    // vektorovy sucin
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0]
        )
    }

    // nahodny vektor v rozsahu
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(rng.random_range(min..max), rng.random_range(min..max), rng.random_range(min..max))
    }

    // funkcia pre nahodny bod v guli
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    // optimalizacia pre spravnu cast gule 
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 { // vektor sucin > 0 => ostry uhol => dobra polgula
            in_unit_sphere
        } else {
            -in_unit_sphere // tupy uhol => otocime vektor
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        let dot = self.dot(normal);
        let refl = *self - (normal * dot * 2.0); 

        refl
    }

}

// Vec3 + Vec3 => vrati novy vector
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
        
    }
}

// Vec3 -  Vec3 => vrati novy vector
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

// nasobenie vectora skalarom
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

// nasobenie vektora vektorom
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2])
    }
}

// delenie vektora skalarom
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

// negacia zmeni smer vectoru
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3{
        Vec3::new(self.e[0] * -1.0, self.e[1] * -1.0, self.e[2] * -1.0)
    }
} 



// alias for better readability
pub type Point3 = Vec3;
pub type Color = Vec3;