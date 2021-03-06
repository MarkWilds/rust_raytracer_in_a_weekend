use std::fmt;
use std::fmt::Write;
use std::ops;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { x: 0_f32, y: 0_f32, z: 0_f32 }
    }

    pub fn one() -> Vec3 {
        Vec3 { x: 1_f32, y: 1_f32, z: 1_f32 }
    }

    pub fn random() -> Vec3 {
        Vec3 { x: thread_rng().gen(), y: thread_rng().gen(), z: thread_rng().gen() }
    }

    pub fn random_min_max(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: thread_rng().gen_range(min..max),
            y: thread_rng().gen_range(min..max),
            z: thread_rng().gen_range(min..max),
        }
    }

    pub fn new_filled(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn reflect(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        *lhs - *rhs * 2f32 * lhs.dot(rhs)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(-uv.dot(n), 1.0);
        let r_out_perp = (*uv + (*n * cos_theta)) * etai_over_etat;
        let r_out_parallel = *n * -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared()));
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        f32::abs(self.x) < f32::EPSILON && f32::abs(self.y) < f32::EPSILON && f32::abs(self.z) < f32::EPSILON
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn normalize(&self) -> Vec3 {
        let l: f32 = self.length();
        Vec3::new_filled(self.x / l, self.y / l, self.z / l)
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new_filled(self.y * rhs.z - self.z * rhs.y,
                         self.z * rhs.x - self.x * rhs.z,
                         self.x * rhs.y - self.y * rhs.x)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.x.to_string().as_str())?;
        f.write_char(' ')?;
        f.write_str(self.y.to_string().as_str())?;
        f.write_char(' ')?;
        f.write_str(self.z.to_string().as_str())?;
        Ok(())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new_filled(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new_filled(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new_filled(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new_filled(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}