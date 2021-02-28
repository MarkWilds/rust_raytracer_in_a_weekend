use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new_zero() -> HitRecord {
        HitRecord {
            p: Vec3::new(),
            n: Vec3::new(),
            t: 0.0,
            front_face: true
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = if outward_normal.dot(&ray.direction) < 0.0 {true} else {false};
        self.n = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool;
}