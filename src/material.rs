use serde::{Deserialize, Serialize};
use crate::metal::Metal;
use crate::lambert::Lambert;
use crate::vec3::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
pub enum Material {
    Metal(Metal),
    Lambert(Lambert),
}

impl Material {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        match self {
            Material::Metal(m) => m.scatter(r, rec),
            Material::Lambert(l) => l.scatter(r, rec),
        }
    }
}