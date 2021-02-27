use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {origin: Vec3::new(), direction: Vec3::new()}
    }

    pub fn new_filled(o: Vec3, d: Vec3) -> Ray {
        Ray {origin: o, direction: d}
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn hit_sphere(&self, center: &Vec3, radius: f32) -> f32 {
        let oc = self.origin - *center;

        let a = self.direction.length_squared();
        let half_b = oc.dot(&self.direction);
        let c = oc.length_squared() - radius * radius;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            -1.0
        } else {
            (-half_b - f32::sqrt(disc)) / a
        }
    }
}