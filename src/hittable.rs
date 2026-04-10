use crate::vec3::{Point3}; 
use crate::ray::{Ray};
use crate::material::{Material};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub t: f64,
    pub normal: Point3,
    pub mat: &'a Material,
}

 
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

