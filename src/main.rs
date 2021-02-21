mod vec3;
mod ray;

use crate::vec3::Vec3;
use crate::ray::Ray;

fn write_header(w: u16, h: u16) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
}

fn write_pixel(color: &Vec3) {
    println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
}

fn ray_color(ray: &Ray) -> Vec3 {
    let norm = ray.direction.normalize();
    let t = norm.y * 0.5_f32 + 1.0_f32;
    let color = Vec3::new_filled(1.0, 1.0, 1.0) * (1.0_f32 - t) + Vec3::new_filled(0.5, 0.7, 1.0) * t;
    color * 255.0
}

fn main() {
    let image_width = 400_u16;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as u16;

    let viewport_height = 2.0;
    let viewport_width = 2.0 * aspect_ratio;
    let focal_point = 1.0;

    let origin = Vec3::new();
    let horizontal = Vec3::new_filled(viewport_width, 0.0 ,0.0);
    let vertical = Vec3::new_filled(0.0, viewport_height ,0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new_filled(0.0, 0.0, focal_point);

    write_header(image_width, image_height);
    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f32 / (image_width - 1) as f32;
            let v = y as f32 / (image_height - 1) as f32;

            let dir = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::new_filled(origin, dir);

            let color = ray_color(&ray);
            write_pixel(&color);
        }
    }
}
