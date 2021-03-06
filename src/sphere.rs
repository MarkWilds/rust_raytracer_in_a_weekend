use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    p: Vec3,
    r: f32,
    material_ref: Rc<dyn Material>
}

impl Sphere {
    pub fn new(p: &Vec3, r: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            p: *p,
            r,
            material_ref: material
        }
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
        hit.material_ref = self.material_ref.clone();

        true
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let s = Vec3::random_min_max(-1.0, 1.0);
        if s.length_squared() < 1.0 {return s;}
    }
}