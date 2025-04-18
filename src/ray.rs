use crate::vector3d::Vector3D;

pub struct Ray {
    point: Vector3D,
    direction: Vector3D,
}

impl Ray {
    pub fn at(self, t: f64) -> Vector3D {
        self.direction.scalar_mul(t) + self.point
    }

    pub fn new(point: Vector3D, direction: Vector3D) -> Self {
        Ray { point, direction }
    }

    pub fn color(self) -> Vector3D {
        let unit_direction: Vector3D = self.direction.normalize();
        let alpha = 0.5 * (unit_direction.y() + 1.0);
        Vector3D::new(1.0, 1.0, 1.0).scalar_mul(1.0 - alpha) + Vector3D::new(0.5, 0.7, 1.0).scalar_mul(alpha)
    }
}
