use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {origin: Vec3::new(), direction: Vec3::new()}
    }

    pub fn new_filled(o: Vec3, d: Vec3) -> Ray {
        Ray {origin: o, direction: d}
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}