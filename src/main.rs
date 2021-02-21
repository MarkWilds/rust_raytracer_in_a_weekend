mod vec3;
use crate::vec3::Vec3;

const WIDTH:u16 = 256;
const HEIGHT:u16 = 256;

fn write_header(w: u16, h: u16) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
}

fn write_pixel(color: &Vec3) {
    println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
}

fn main() {
    write_header(WIDTH, HEIGHT);
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let color = Vec3::new_filled(255.0 * x as f32 / (WIDTH - 1) as f32,
                                          255.0 * y as f32 / (HEIGHT - 1) as f32,
                                          255.0 * 0.25);
            write_pixel(&color);
        }
    }
}
