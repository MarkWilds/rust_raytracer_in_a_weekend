mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::hittable::{Hittable, HitRecord};
use std::borrow::Borrow;
use rand::Rng;

fn write_header(w: u16, h: u16) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
}

fn write_pixel(unit_color: &Vec3, samples_per_pixel: u8) {
    let color = *unit_color * (1.0 / samples_per_pixel as f32);
    println!("{} {} {}",
             (f32::clamp(color.x, 0.0, 0.999) * 256.0) as i32,
             (f32::clamp(color.y, 0.0, 0.999) * 256.0) as i32,
             (f32::clamp(color.z, 0.0, 0.999) * 256.0) as i32);
}

fn get_ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>) -> Vec3 {
    let mut hit = HitRecord::new_zero();
    if hit_world(objects, ray, &mut hit) {
        let n = hit.n;
        return (Vec3::new_filled(n.x, n.y, n.z) + Vec3::one()) * 0.5
    }

    let norm = ray.direction.normalize();
    let t = 0.5 * (norm.y + 1.0);
    let color = Vec3::one() * (1.0_f32 - t) + Vec3::new_filled(0.5, 0.7, 1.0) * t;
    color
}

fn hit_world(objects: &Vec<Box<dyn Hittable>>, ray: &Ray, hit: &mut HitRecord) -> bool {
    let mut hit_anything = false;
    let mut closest_so_far = f32::MAX;

    for object in objects {
        if object.hit(ray, 0.0, closest_so_far, hit) {
            hit_anything = true;
            closest_so_far = hit.t;
        }
    }

    hit_anything
}

fn main() {
    let samples_per_pixel = 50_u8;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400_u16;
    let image_height = (image_width as f32 / aspect_ratio) as u16;

    let camera = Camera::new(aspect_ratio);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new_filled(0.0, 0.0, -1.0).borrow(), 0.5)),
        Box::new(Sphere::new(Vec3::new_filled(0.0, -100.5, -1.0).borrow(), 100.0))
    ];

    write_header(image_width, image_height);
    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut unit_color = Vec3::new();
            for _ in 0..samples_per_pixel {
                let rx: f32 = rand::thread_rng().gen();
                let ry: f32 = rand::thread_rng().gen();
                let u = (x as f32 + rx) / (image_width - 1) as f32;
                let v = (y as f32 + ry) / (image_height - 1) as f32;

                let ray = camera.get_ray_at_uv(u, v);
                unit_color += get_ray_color(&ray, &world);
            }
            write_pixel(&unit_color, samples_per_pixel);
        }
    }
}
