use crate::{hittable::Hittable, interval::Interval, ray::Ray};

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
    fn hit(&self, interval: &Interval, ray: &Ray) -> Option<crate::hittable::HitInfo> {
        for object in &self.hittables {
            if let Some(info) = object.hit(&interval, ray) {
                return Some(info);
            }
        }
        None
    }
}