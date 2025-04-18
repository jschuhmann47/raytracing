use std::ops::{Add, Mul, Sub};

use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    pub fn to_color(self) -> Color {
        // Assuming that x,y,z are in the range of [0;1]
        Color::new(
            (self.x * 255.999).trunc() as u8, 
            (self.y * 255.999).trunc() as u8, 
            (self.z * 255.999).trunc() as u8
        )
    }

    pub fn scalar_mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        Self { x: self.x / length, y: self.y / length, z: self.z /length }
    }

    fn length(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    } 
}
