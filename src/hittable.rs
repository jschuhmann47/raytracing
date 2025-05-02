use crate::{interval::Interval, ray::Ray, vector3d::Vector3D};

pub trait Hittable {
    fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitInfo>;
}

pub struct HitInfo {
    point: Vector3D,
    normal: Vector3D,
    t: f64,
}

impl HitInfo {
    pub fn new(point: Vector3D, normal: Vector3D, t: f64) -> Self {
        Self { point, normal, t }
    }

    pub fn normal(&self) -> Vector3D {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3D) {
        let is_facing_front = ray.direction().dot_product(outward_normal) < 0.0;
        self.normal = if is_facing_front {
            self.normal
        } else {
            self.normal.scalar_mul(-1.0)
        }
    }
}
