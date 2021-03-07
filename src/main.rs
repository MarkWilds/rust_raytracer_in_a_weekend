mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::*;
use crate::camera::Camera;
use crate::hittable::{Hittable, HitRecord};
use std::borrow::Borrow;
use rand::{Rng, thread_rng};
use crate::material::{Lambertian, DummyMaterial, Metal, Dielectric};
use std::rc::Rc;

fn write_header(w: u16, h: u16) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
}

fn write_pixel(unit_color: &Vec3, samples_per_pixel: u16) {
    let r= f32::sqrt(unit_color.x * (1.0 / samples_per_pixel as f32));
    let g= f32::sqrt(unit_color.y * (1.0 / samples_per_pixel as f32));
    let b= f32::sqrt(unit_color.z * (1.0 / samples_per_pixel as f32));

    println!("{} {} {}",
             (f32::clamp(r, 0.0, 0.999) * 256.0) as i32,
             (f32::clamp(g, 0.0, 0.999) * 256.0) as i32,
             (f32::clamp(b, 0.0, 0.999) * 256.0) as i32);
}

fn get_ray_color(ray: &Ray, objects: &Vec<Box<dyn Hittable>>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    let dummy_material = Rc::new(DummyMaterial {});
    let mut hit = HitRecord::new_zero(dummy_material);
    if hit_world(objects, ray, &mut hit) {

        let mut new_ray = Ray::new();
        let mut color = Vec3::new();
        if hit.material_ref.scatter(ray, &hit, &mut color, &mut new_ray) {
            return color * get_ray_color(&new_ray, objects, depth - 1);
        }

        return Vec3::new();
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
        if object.hit(ray, 0.001, closest_so_far, hit) {
            hit_anything = true;
            closest_so_far = hit.t;
        }
    }

    hit_anything
}

fn create_scene(objects: &mut Vec<Box<dyn Hittable>>) {
    let ground_mat = Rc::new(Lambertian {albedo: Vec3::new_filled(0.5, 0.5, 0.5)});
    objects.push(Box::new(Sphere::new(Vec3::new_filled(0.0, -1000.0,0.0), 1000.0, ground_mat.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = thread_rng().gen_range(0.0..1.0);
            let center = Vec3::new_filled(a as f32 + 0.9 * thread_rng().gen_range(0.0..1.0),
            0.2,
            b as f32 + 0.9 * thread_rng().gen_range(0.0..1.0));

            if (center - Vec3::new_filled(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Vec3::random() * Vec3::random();
                    let mat = Rc::new(Lambertian {albedo: color});
                    objects.push(Box::new(Sphere::new(center, 0.2, mat.clone())));
                } else if choose_mat < 0.5 {
                    let color = Vec3::random_min_max(0.5, 1.0);
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let mat = Rc::new(Metal {albedo: color, fuzz });
                    objects.push(Box::new(Sphere::new(center, 0.2, mat.clone())));
                } else {
                    let mat = Rc::new(Dielectric{ir: 1.5});
                    objects.push(Box::new(Sphere::new(center, 0.2, mat.clone())));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric{ir: 1.5});
    objects.push(Box::new(Sphere::new(Vec3::new_filled(0.0, 1.0,0.0), 1.0, mat1.clone())));

    let mat2 = Rc::new(Lambertian {albedo: Vec3::new_filled(0.4, 0.2, 0.1)});
    objects.push(Box::new(Sphere::new(Vec3::new_filled(-4.0, 1.0,0.0), 1.0, mat2.clone())));

    let mat3 = Rc::new(Metal {albedo: Vec3::new_filled(0.7, 0.6, 0.5), fuzz: 0.0});
    objects.push(Box::new(Sphere::new(Vec3::new_filled(4.0, 1.0,0.0), 1.0, mat3.clone())));
}

fn main() {
    let max_depth = 50;
    let samples_per_pixel = 500_u16;
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200_u16;
    let image_height = (image_width as f32 / aspect_ratio) as u16;

    let lookfrom = Vec3::new_filled(13.0,2.0,3.0);
    let lookat = Vec3::new_filled(0.0,0.0,0.0);
    let vup = Vec3::new_filled(0.0,1.0,0.0);
    let camera = Camera::new(lookfrom, lookat, vup, 90.0,
                             aspect_ratio, 0.1, 10.0);

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    create_scene(&mut world);

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
                unit_color += get_ray_color(&ray, &world, max_depth);
            }
            write_pixel(&unit_color, samples_per_pixel);
        }
    }
}
