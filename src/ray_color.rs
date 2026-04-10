use crate::object_list::{Object_list};
use crate::ray::{Ray};
use crate::vec3::{Color};
use crate::hittable::{Hittable};


pub fn ray_color(r: &Ray, s: &Object_list, depth: i32) -> Color {
    // ak sme narazili na limit odrazov, svetlo zaniklo
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = s.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered_ray)) = rec.mat.scatter(r, &rec) {
            // rekuzia kvoli bounce t.j kolko odrazov sa vykona pre kazdy pixel
            return attenuation * ray_color(&scattered_ray, s, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0); // pohltili sme svetlo, luc sa neodrazil von 
    }

    // farba oblohy ked nepide na hit
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}