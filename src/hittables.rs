use crate::{hittable::Hittable, ray::Ray};

pub struct Hittables {
    hittables: Vec<Box<dyn Hittable>>
}

impl Hittables {
    pub fn new() -> Self {
        Self { hittables: Vec::new() }
    }

    pub fn add(&mut self,element: Box<dyn Hittable>) {
        self.hittables.push(element);
    }
}

impl Hittable for Hittables {
    fn hit(&self, t_min: f64, t_max: f64, ray: &Ray) -> Option<crate::hittable::HitInfo> {
        for object in &self.hittables {
            if let Some(info) = object.hit(t_min, t_max, ray) {
                return Some(info);
            }
        }
        None
    }
}