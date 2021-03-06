use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f64::consts::PI;
use crate::sphere::random_in_unit_disk;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32,
               aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov * PI as f32 / 180.0;
        let h = (theta/2.0f32).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        Camera {
            origin, horizontal, vertical, lower_left_corner,
            w, u, v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray_at_uv(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new_filled(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}