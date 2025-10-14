use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use crate::utils::{rand_f64, rand_f64_in};

pub type Point3 = Vec3;

const S: f64 = 1e-8;

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn from(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn from_x(x: f64) -> Self {
        Self { x, y: 0.0, z: 0.0 }
    }

    pub const fn from_y(y: f64) -> Self {
        Self { x: 0.0, y, z: 0.0 }
    }

    pub const fn from_z(z: f64) -> Self {
        Self { x: 0.0, y: 0.0, z }
    }

    pub const fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub const fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub const fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    #[inline(always)]
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    #[inline(always)]
    pub fn cross(u: Vec3, v: Vec3) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn stable_length(&self) -> f64 {
        self.stable_length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn stable_length_squared(&self) -> f64 {
        let max = self.x.abs().max(self.y.abs()).max(self.z.abs());
        if max == 0.0 {
            0.0
        } else {
            let x = self.x / max;
            let y = self.y / max;
            let z = self.z / max;
            max * max * (x * x + y * y + z * z)
        }
    }

    pub fn near_zero(&self) -> bool {
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }

    #[inline(always)]
    pub fn random() -> Self {
        Self {
            x: rand_f64(),
            y: rand_f64(),
            z: rand_f64(),
        }
    }

    #[inline(always)]
    pub fn random_in(min: f64, max: f64) -> Self {
        Self {
            x: rand_f64_in(min, max),
            y: rand_f64_in(min, max),
            z: rand_f64_in(min, max),
        }
    }

    #[inline(always)]
    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }

    #[inline(always)]
    pub fn stable_unit_vector(v: Self) -> Self {
        v / v.stable_length()
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_in(-1.0, 1.0);
            if p.stable_length_squared() <= 1.0 {
                return Self::stable_unit_vector(p);
            }
        }
    }

    #[inline(always)]
    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline(always)]
    pub fn reflect(v: Self, n: Self) -> Self {
        v - 2.0 * Self::dot(v,n) * n
    }

    #[inline(always)]
    pub fn refract(uv: Self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = Self::dot(-uv, n).min(1.0);
        let perp = etai_over_etat * (uv + cos_theta * n);
        let parallel = -(1.0 - perp.length_squared()).abs().sqrt() * n;
        perp + parallel
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
