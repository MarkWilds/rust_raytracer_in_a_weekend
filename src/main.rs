mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::hittable::{Hittable, HitRecord};
use std::borrow::Borrow;

fn write_header(w: u16, h: u16) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
}

fn write_pixel(color: &Vec3) {
    println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
}

fn get_ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Vec3 {
    let mut hit = HitRecord::new_zero();
    let mut temp_hit = HitRecord::new_zero();
    temp_hit.t = f32::MAX;

    for object in objects {
        if object.hit(ray, 0.0, temp_hit.t, &mut hit) {
            if  hit.t < temp_hit.t {
                temp_hit = hit;
            }
        }
    }

    if temp_hit.t == f32::MAX || temp_hit.t < 0.0 {
        let norm = ray.direction.normalize();
        let t = 0.5 * (norm.y + 1.0);
        let color = Vec3::new_filled(1.0, 1.0, 1.0) * (1.0_f32 - t) + Vec3::new_filled(0.5, 0.7, 1.0) * t;
        return color * 255.0
    }

    let n = temp_hit.n;
    Vec3::new_filled(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5 * 255.0
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

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new_filled(0.0, 0.0, -1.0).borrow(), 0.5))
    ];

    write_header(image_width, image_height);
    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f32 / (image_width - 1) as f32;
            let v = y as f32 / (image_height - 1) as f32;

            let dir = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::new_filled(origin, dir);

            let color = get_ray_color(&ray, &objects);
            write_pixel(&color);
        }
    }
}
