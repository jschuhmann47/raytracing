// Values between 0 and 1
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z }
    }
    pub fn to_line(&self) -> String {
        format!("{} {} {}\n", self.x, self.y, self.z)
    }

    pub fn to_u8_range(&self) -> Self {
        Self {
            x: (self.x * 255.999).trunc(),
            y: (self.y * 255.999).trunc(),
            z: (self.z * 255.999).trunc(),
        }
    }
}
