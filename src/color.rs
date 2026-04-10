use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    // ziskame zlozky farieb 0-1
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // prepocet na rozsah 0-255
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    // check ak je farba zaporna vykreslime ruzovu tkzv. error farba
    if(ir < 0 || ig < 0 || ib < 0) {
        println!("{} {} {}", 255, 0, 255);
        return
    }

    // vypis do ppm formatu
    println!("{} {} {}", ir, ig, ib);
}