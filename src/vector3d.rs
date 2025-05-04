use std::ops::{Add, Mul, Sub};

use crate::{color::Color, interval::Interval};

#[derive(Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    /// Works assuming that x, y and z are in the range of ``[0;1]``
    pub fn to_color(self) -> Color {
        let intensity = Interval::new(0.0, 0.999);
        Color::new(
            (intensity.clamp(self.x) * 256.0) as u8,
            (intensity.clamp(self.y) * 256.0) as u8,
            (intensity.clamp(self.z) * 256.0) as u8,
        )
    }

     /// Works assuming that x, y and z are in the range of ``[0;255]``
     pub fn as_color(self) -> Color {
        Color::new(
            (self.x) as u8,
            (self.y) as u8,
            (self.z) as u8,
        )
    }

    pub fn scalar_mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn scalar_div(self, t: f64) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }

    pub fn scalar_sum(self, t: f64) -> Self {
        Self {
            x: self.x + t,
            y: self.y + t,
            z: self.z + t,
        }
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn dot_product(self, vec: Vector3D) -> f64 {
        let mul = self * vec;
        mul.x + mul.y + mul.z
    }

    /// Equivalent of doing `vector.dot_product(vector)` but faster
    pub fn squared_length(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product_ok() {
        let some_vec = Vector3D::new(2.0, -2.0, 3.0);
        let another_vec = Vector3D::new(4.0, 1.0, 2.0);
        let result = some_vec.dot_product(another_vec);
        assert_eq!(result, 12.0);
    }
}
