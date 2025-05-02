use crate::{
    color::Color, hittable::Hittable, hittables::Hittables, ppm::PpmImage, ray::Ray,
    vector3d::Vector3D, viewport::Viewport,
};

pub struct Camera {
    aspect_radio: f64,
    width: u32,

    height: u32,
    camera_center: Vector3D,
    first_pixel_location: Vector3D,
    delta_u: Vector3D,
    delta_v: Vector3D,
}

impl Camera {
    pub fn render(&self, world: Hittables) -> PpmImage {
        let mut fields: Vec<Color> = Vec::with_capacity((self.width * self.height) as usize);
        for j in 1..=self.height {
            for i in 1..=self.width {
                let pixel_center = self.first_pixel_location
                    + (self.delta_u.scalar_mul(i.into()))
                    + self.delta_v.scalar_mul(j.into());
                let direction = pixel_center - self.camera_center;
                let new_ray = Ray::new(self.camera_center, direction);
                let color = new_ray.color(&world);
                fields.push(color);
            }
        }
        PpmImage::new(self.width, self.height, 255, fields)
    }

    pub fn initialize(aspect_radio: f64, width: u32) -> Self {
        // TODO unify width and height types (f64 or u32)
        let focal_length = 1.0;
        let camera_center = Vector3D::new(0.0, 0.0, 0.0);
        let height = (f64::from(width) / aspect_radio) as u32;
        let viewport_height = 2.0;
        let viewport = Viewport::new(
            viewport_height * f64::from(width) / f64::from(height),
            viewport_height,
        );

        let viewport_upper_left = camera_center
            - Vector3D::new(0.0, 0.0, focal_length)
            - viewport.u().scalar_div(2.0)
            - viewport.v().scalar_div(2.0);

        let delta_u = viewport.u().scalar_div(f64::from(width));
        let delta_v = viewport.v().scalar_div(f64::from(height));

        let pixel00_location = viewport_upper_left + (delta_u + delta_v).scalar_div(2.0);
        Self {
            aspect_radio,
            width,
            height,
            camera_center,
            delta_u,
            delta_v,
            first_pixel_location: pixel00_location,
        }
    }
}
