use crate::{hittable::{HitInfo, Hittable}, interval::Interval, ray::Ray, vector3d::Vector3D};

pub struct Sphere {
    center: Vector3D,
    radius: f64
}
impl Sphere {

    pub fn new(center: Vector3D, radius: f64) -> Self {
        Self { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, interval: &Interval, ray: &Ray) -> Option<HitInfo> {
        let oc = self.center - ray.origin();
        let a = ray.direction().squared_length();
        let h = ray.direction().dot_product(oc);
        let c = oc.dot_product(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        } 
        let mut root = (h-discriminant.sqrt())/a;
        if !interval.contains_inclusive(root) {
            root = (h+discriminant.sqrt())/a;
            if !interval.contains(root) {
                return None;
            }
        }
        let ray_at_root = ray.at(root);
        let outward_normal = (ray_at_root - self.center).scalar_div(self.radius);
        let mut hit_info = HitInfo::new(
            ray_at_root,
            outward_normal,
            root
        );
        hit_info.set_face_normal(ray, outward_normal);
        Some(hit_info)
    }
}