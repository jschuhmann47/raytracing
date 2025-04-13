use crate::vector3d::Vector3D;

struct Ray {
    point: Vector3D,
    direction: Vector3D,
}

impl Ray {
    pub fn at(self, t: f64) -> Vector3D {
        self.direction.scalar_mul(t) + self.point
    }
}
