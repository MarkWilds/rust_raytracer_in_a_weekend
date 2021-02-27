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

    pub fn hit_sphere(&self, center: &Vec3, radius: f32) -> bool {
        let oc = self.origin - *center;

        let a = self.direction.length_squared();
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.length_squared() - radius * radius;
        let disc = b * b - 4.0 * a * c;
        disc > 0.0
    }
}