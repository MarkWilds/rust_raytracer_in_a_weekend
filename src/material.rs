use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::sphere::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        let mut dir = hit.n + random_in_unit_sphere().normalize();

        if dir.near_zero() {
            dir = hit.n;
        }

        scattered_ray.origin = hit.p;
        scattered_ray.direction = dir;
        color.x = self.albedo.x;
        color.y = self.albedo.y;
        color.z = self.albedo.z;

        true
    }
}

pub struct Metal {
    pub albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        scattered_ray.origin = hit.p;
        scattered_ray.direction = Vec3::reflect(&ray_in.direction.normalize(), &hit.n);
        color.x = self.albedo.x;
        color.y = self.albedo.y;
        color.z = self.albedo.z;

        scattered_ray.direction.dot(&hit.n) > 0f32
    }
}

pub struct DummyMaterial {}

impl Material for DummyMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        unimplemented!()
    }
}