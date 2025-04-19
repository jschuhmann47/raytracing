use crate::{ray::Ray, vector3d::Vector3D};

pub struct Sphere {
    center: Vector3D,
    radius: f64
}
impl Sphere {

    pub fn new(center: Vector3D, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn is_ray_hitting(self, ray: &Ray) -> bool {
        let oc = self.center - ray.origin();
        // a, b and c come from the quadratic formula
        let a = ray.direction().dot_product(ray.direction());
        let b = -2.0 * ray.direction().dot_product(oc);
        let c = oc.dot_product(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}