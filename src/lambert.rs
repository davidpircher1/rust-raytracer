use serde::{Deserialize, Serialize};
use crate::hittable::{HitRecord};
use crate::vec3::{Color,Vec3};
use crate::ray::{Ray};

#[derive(Deserialize, Debug, Serialize)]
pub struct Lambert {
    pub color: Color,
}

impl Lambert {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // smer odrazu (nahodny)
        let target = rec.p + rec.normal + Vec3::random_in_hemisphere(rec.normal);

        // smerovy vektor => ciel - start 
        let direction = target - rec.p; 

        // vytvorime luc s nasim smerom a novym smerovym vektorom
        let target_ray = Ray::new(rec.p, direction);

        Some((self.color, target_ray))
    }
}
