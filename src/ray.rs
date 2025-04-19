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
        let sphere_origin = Vector3D::new(0.0, 0.0, -1.0);
        let test_sphere = Sphere::new(sphere_origin, 0.5);
        if let Some(t) = test_sphere.hit(&self) {
            let normal = (self.at(t) - sphere_origin).normalize();
            return normal.scalar_sum(1.0).scalar_mul(0.5).to_color()
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
