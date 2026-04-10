mod vec3;
mod ray;
mod camera;
mod color;
mod sphere;
mod hittable;
mod object_list;
mod ray_color;
mod material;
mod lambert;
mod metal;

use vec3::{Color};
use object_list::{Object_list};
use camera::Camera;
use color::write_color;
use sphere::{Sphere};
use ray_color::{ray_color};
use crate::vec3::{Point3};
use rand::prelude::*;
use crate::lambert::{Lambert};
use crate::metal::{Metal};
use crate::material::Material;
use std::fs;
use serde_json;




fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 720;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 256; // samples pre antialiasing - priemer pre pixel => cim viac, tym dlhsie no
    let mut rng = rand::rng();  // nahodne cisla gen

    let mut scene = Object_list::new();

    let gula_tri = Box::new(Sphere::new(100000.0, Point3::new(0.5,-100001.0,-2.3), Material::Lambert(Lambert { color: Color::new(0.1, 0.9, 0.1)})));

    scene.add(gula_tri);

    // read data
    let data = fs::read_to_string("scena.json")
        .expect("chyba pri jsone");

let gule: Vec<Sphere> = serde_json::from_str(&data).expect("chyba pri jsone");
for g in gule {
    scene.add(Box::new(g));
}




    let cam = Camera::new(ASPECT_RATIO);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_sum_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                // posun pixelu o random kus
                let u = (i as f64 + rng.random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                // ziskanie luca pre suradnice ako pixely
                let r = cam.get_ray(u, v);
                
                // ray color vrati priemer farieb (antialiasing)
                pixel_sum_color += ray_color(&r, &scene, 4);
            }

            // vydelime poctom samples (antialiasing)
            write_color(pixel_sum_color / SAMPLES_PER_PIXEL as f64);
        }
    }
}

// prikaz na spustenie -> cargo run --quiet | Out-File -FilePath img.ppm -Encoding ascii ; ./imgView.exe img.ppm