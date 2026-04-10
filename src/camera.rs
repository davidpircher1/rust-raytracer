use crate::vec3::{Vec3, Point3}; 
use crate::ray::{Ray};

pub struct Camera {
    pub origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3, 
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height: f64 = 2.0;
        let viewport_width:f64 = aspect_ratio * viewport_height;
        let focal_length: f64= 1.0; // ohinskova vzidalenost

        let origin = Point3::new(0.0, 0.0, 0.0); // bod, kde je nasa kamera
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        
        // lavy dolny roh suradnice, este posun o ohniskovu vzdialenost
        let lower_left_corner = origin 
            - horizontal / 2.0 
            - vertical / 2.0 
            - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }        
    }

    // luc pre konkretny pixel u, v su od 0.0 do 1.0
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin // self.origin vypocet pre hocijaku poziciu
        )
    }
}