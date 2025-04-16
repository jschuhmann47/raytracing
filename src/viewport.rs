use crate::vector3d::Vector3D;

pub struct Viewport {
    viewport_u: Vector3D,
    viewport_v: Vector3D,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            viewport_u: Vector3D::new(width, 0.0, 0.0),
            viewport_v: Vector3D::new(0.0, -height, 0.0),
        }
    }

    pub fn viewpoint_u(&self) -> Vector3D {
        self.viewport_u
    }

    pub fn viewpoint_v(&self) -> Vector3D {
        self.viewport_v
    }
}
