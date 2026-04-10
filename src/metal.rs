use serde::{Deserialize, Serialize};
use crate::hittable::{HitRecord};
use crate::vec3::{Color,Vec3};
use crate::ray::{Ray};

#[derive(Deserialize, Debug, Serialize)]
pub struct Metal {
    pub color: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // smer luca ktory prichadza v nornovanom tvare
        let unit_direction = r.direction.unit_vector();
        
        // vypocitame jeho zrkadlovu verziu
        let reflected = Vec3::reflect(&unit_direction, rec.normal);

        // pridame nejaky fuzz, ze to robi kov kovom xd
        let scatter_direction = reflected + Vec3::random_in_hemisphere(rec.normal) * self.fuzz;
        let scattered_ray = Ray::new(rec.p, scatter_direction);

        // pokial je vektor sucin > 0 => ostry uhol => luc sa odraza von z gule
        if scattered_ray.direction.dot(rec.normal) > 0.0 {
            Some((self.color, scattered_ray))
        } else {
            // tupy uhol => luc sa odraze do gule
            None 
        }   

    }
}
