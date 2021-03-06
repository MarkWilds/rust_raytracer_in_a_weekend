use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::sphere::random_in_unit_sphere;
use rand::{thread_rng, Rng};

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
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        scattered_ray.origin = hit.p;
        scattered_ray.direction = Vec3::reflect(&ray_in.direction.normalize(), &hit.n)
        + random_in_unit_sphere() * self.fuzz;
        color.x = self.albedo.x;
        color.y = self.albedo.y;
        color.z = self.albedo.z;

        scattered_ray.direction.dot(&hit.n) > 0f32
    }
}

pub struct Dielectric {
    pub ir: f32
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        color.x = 1.0;
        color.y = 1.0;
        color.z = 1.0;

        let refraction_ratio = if hit.front_face {1.0 / self.ir} else {self.ir};
        let unit_dir = ray_in.direction.normalize();

        let cos_theta = f32::min(-unit_dir.dot(&hit.n), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let direction = if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > thread_rng().gen_range(0.0..1.0)
        {Vec3::reflect(&unit_dir, &hit.n)}
        else
        {Vec3::refract(&unit_dir, &hit.n, refraction_ratio)};

        scattered_ray.origin = hit.p;
        scattered_ray.direction = direction;

        true
    }
}

pub struct DummyMaterial {}

impl Material for DummyMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, color: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        unimplemented!()
    }
}

fn reflectance(cos: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powf(1.0 - cos, 5.0)
}