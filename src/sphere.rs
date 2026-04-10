use crate::vec3::{Vec3, Point3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
//use crate::material::{Material};
use crate::material::{Material};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sphere {
    radius: f64,
    center: Point3,
   pub mat: Material,
}

impl Sphere {
    pub fn new(r: f64, c: Point3, m: Material) -> Sphere{
        Sphere {
            radius: r,
            center: c,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a:f64 = Vec3::dot(ray.direction,ray.direction);
        let b:f64 = 2.0 * Vec3::dot(ray.direction, ray.origin - self.center);
        let c:f64 = Vec3::dot(ray.origin - self.center, ray.origin - self.center) - self.radius*self.radius;

        let discriminant:f64 = (b*b) - (4.0 *a*c); 
        // ziadne vysledky, cize ziadny dotyk s gulou
        if discriminant < 0.0 {
            return None;
        } 

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-b - sqrt_discriminant) / (2.0 * a); // vysledky nasho parametru t

        if (root <= t_min || root >= t_max) { // 
            root = (-b + sqrt_discriminant) / (2.0 * a); // druhy vysledok parametru t
            if (root <= t_min || root >= t_max) {
                return None;
            }
        }
        
        Some(HitRecord {
            p: ray.at(root),
            t: root,
            normal: Vec3::unit_vector(ray.at(root) - self.center), // normala, normalizovana na 1kovy vektor
            mat: &self.mat, // farbicka gulicky
        })

    }
}

