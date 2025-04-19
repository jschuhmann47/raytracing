use crate::{ray::Ray, vector3d::Vector3D};

pub trait Hittable {
    fn hit(self, t_min: f64, t_max: f64, ray: &Ray) -> Option<HitInfo>;
}

pub struct HitInfo {
    point: Vector3D,
    normal: Vector3D,
    t: f64
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
}