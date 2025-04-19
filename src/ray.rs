use crate::{color::Color, sphere::Sphere, vector3d::Vector3D};

pub struct Ray {
    origin: Vector3D,
    direction: Vector3D,
}

impl Ray {
    pub fn at(self, t: f64) -> Vector3D {
        self.direction.scalar_mul(t) + self.origin
    }

    pub fn new(point: Vector3D, direction: Vector3D) -> Self {
        Ray { origin: point, direction }
    }

    pub fn color(self) -> Color {
        let test_sphere = Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5);
        if test_sphere.is_ray_hitting(&self) {
            return Color::new(255, 0, 0);
        }
        let unit_direction: Vector3D = self.direction.normalize();
        let alpha = 0.5 * (unit_direction.y() + 1.0);
        let result = Vector3D::new(1.0, 1.0, 1.0).scalar_mul(1.0 - alpha) + Vector3D::new(0.5, 0.7, 1.0).scalar_mul(alpha);
        result.to_color()
    }

    pub fn origin(&self) -> Vector3D {
        self.origin
    }

    pub fn direction(&self) -> Vector3D {
        self.direction
    }
}
