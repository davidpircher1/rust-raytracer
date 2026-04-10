use crate::hittable::{HitRecord, Hittable};
use crate::ray::{Ray}; 

pub struct Object_list {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Object_list {

    pub fn new() -> Self {
        Self { list: Vec::new(), }
    }

    pub fn add(&mut self, hit: Box<dyn Hittable>) {
        self.list.push(hit);
    }
}

impl Hittable for Object_list {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut hit_anything:Option<HitRecord> = None;

        for object in &self.list {
            let hit_result = object.hit(ray, t_min, closest_t);

            match hit_result {
                // zasah do cierneho
                Some(rec) => {
                    closest_t = rec.t; // meniem nasu hranicu na blizsiu 
                    hit_anything = Some(rec); // najblizsi zasah aztial
                },

                None => {
                    // nic sme netrafili 
                }
            }
        }

        hit_anything
    }
}

