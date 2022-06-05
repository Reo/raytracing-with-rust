use std::ops;
use crate::utility::{unif_rng, PI};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3d {
    x:f64,
    y:f64,
    z:f64,
}

impl Vec3d {
    pub fn zero() -> Self { Vec3d {x:0.0, y:0.0, z:0.0} }
    pub fn rand_unit() -> Self {
        let u = unif_rng(-1.0, 1.0);
        let theta = unif_rng(0.0, 2.0*PI);
        Vec3d {
            x: (1.0 - u*u).sqrt() * theta.cos(),
            y: (1.0 - u*u).sqrt() * theta.sin(),
            z: u
        }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self { Vec3d {x:x, y:y, z:z} }
    pub fn len(self) -> f64 { (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() }

    pub fn x(self) -> f64 { self.x }
    pub fn y(self) -> f64 { self.y }
    pub fn z(self) -> f64 { self.z }
    pub fn r(self) -> f64 { self.x }
    pub fn g(self) -> f64 { self.y }
    pub fn b(self) -> f64 { self.z }
}
pub fn dot(u: Vec3d, v: Vec3d) -> f64 { u.x * v.x + u.y * v.y + u.z * v.z }
pub fn cross(u: Vec3d, v: Vec3d) -> Vec3d {
    Vec3d {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x
    }
}

pub fn normalise(v: Vec3d) -> Vec3d {
    v/v.len()
}


/* NEG */
impl ops::Neg for Vec3d {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y, z: -self.z }
    }
}

/* SUM */
impl ops::Add for Vec3d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl ops::AddAssign for Vec3d {
    fn add_assign (&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for Vec3d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl ops::SubAssign for Vec3d {
    fn sub_assign (&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

/* MUL */
impl ops::Mul<f64> for Vec3d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}

impl ops::MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl ops::Mul<Vec3d> for f64 {
    type Output = Vec3d;
    fn mul (self, vec: Vec3d) -> Self::Output { vec * self }
}

/* DIV */
impl ops::Div<f64> for Vec3d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self {x: self.x / scalar, y: self.y / scalar, z: self.z / scalar}
    }
}

impl ops::DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// aliases
pub use Vec3d as RGBcol;
pub use Vec3d as Point3d;

fn main() {
    // aliases and addition
    assert_eq!(Vec3d {x: 1.0, y: 2.0, z: 3.0}, Vec3d {x: 1.0, y: 1.0, z: 1.0} + Vec3d {x: 0.0, y: 1.0, z: 2.0});
    assert_eq!(RGBcol {x: 1.0, y: 2.0, z: 3.0}, RGBcol {x: 1.0, y: 1.0, z: 1.0} + RGBcol {x: 0.0, y: 1.0, z: 2.0});
    assert_eq!(Point3d {x: 1.0, y: 2.0, z: 3.0}, Point3d {x: 1.0, y: 1.0, z: 1.0} + Point3d {x: 0.0, y: 1.0, z: 2.0});
    // mul
    assert_eq!(Vec3d {x: 1.0, y: 2.0, z: 3.0}, Vec3d {x: 0.5, y: 1.0, z: 1.5} * 2.0);
    assert_eq!(Vec3d {x: 1.0, y: 2.0, z: 3.0}, 2f64 * Vec3d {x: 0.5, y: 1.0, z: 1.5});
    let mut v = Vec3d { x: 1.0, y: 2.0, z: 3.0};
    v *= 5.0;
    assert_eq!(Vec3d {x: 5.0, y: 10.0, z: 15.0}, v);
    // div
    assert_eq!(Vec3d {x: 1.0, y: 2.0, z: 3.0}, Vec3d {x: 2.0, y: 4.0, z: 6.0} / 2.0);
    let mut v = Vec3d { x: 2.0, y: 8.0, z: 10.0};
    v /= 2.0;
    assert_eq!(Vec3d {x: 1.0, y: 4.0, z: 5.0}, v);
    // len
    let v = Vec3d { x: 9.0, y: 12.0, z: 20.0 };
    assert_eq!(v.len(), 25.0);
    // unit vector
    // neg
    assert_eq!(Vec3d {x: -9.0, y: -12.0, z: -20.0}, -v);
    println!("{:?}", v);
}

