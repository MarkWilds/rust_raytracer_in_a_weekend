use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = 2.0 * aspect_ratio;
        let focal_point = 1.0;

        let origin = Vec3::new();
        let horizontal = Vec3::new_filled(viewport_width, 0.0 ,0.0);
        let vertical = Vec3::new_filled(0.0, viewport_height ,0.0);
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new_filled(0.0, 0.0, focal_point);

        Camera {
            origin, horizontal, vertical, lower_left_corner
        }
    }

    pub fn get_ray_at_uv(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new_filled(self.origin, dir)
    }
}