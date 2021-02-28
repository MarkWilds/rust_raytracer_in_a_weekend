use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    p: Vec3,
    r: f32
}

impl Sphere {
    pub fn new(p: &Vec3, r: f32) -> Sphere {
        Sphere { p: *p, r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut HitRecord) -> bool {
        let oc = ray.origin - self.p;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.r * self.r;
        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return false
        }

        let sqrtd = f32::sqrt(disc);
        let root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        hit.t = root;
        hit.p = ray.at(hit.t);
        let n = (hit.p - self.p) * (1.0 / self.r);
        hit.set_face_normal(&ray, &n);

        true
    }
}