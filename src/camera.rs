use crate::{
    color::Color, hittable::Hittable, hittables::Hittables, ppm::PpmImage, ray::Ray, utils::{self, random_double}, vector3d::Vector3D, viewport::Viewport
};

pub struct Camera {
    aspect_radio: f64,
    width: u32,
    samples_per_pixel: u32,

    height: u32,
    camera_center: Vector3D,
    first_pixel_location: Vector3D,
    delta_u: Vector3D,
    delta_v: Vector3D,
    pixel_samples_scale: f64
}

impl Camera {
    pub fn render(&self, world: Hittables) -> PpmImage {
        let mut fields: Vec<Color> = Vec::with_capacity((self.width * self.height) as usize);
        for j in 1..=self.height {
            for i in 1..=self.width {
                let mut color_vec = Vector3D::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let new_ray = self.get_ray(i, j);
                    color_vec = color_vec + new_ray.color(&world).to_vector3d();
                }
                fields.push(color_vec.scalar_mul(self.pixel_samples_scale).as_color());
            }
        }
        PpmImage::new(self.width, self.height, 255, fields)
    }

    pub fn initialize(aspect_radio: f64, width: u32, samples_per_pixel: u32) -> Self {
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
            samples_per_pixel,
            pixel_samples_scale: 1.0 / f64::from(samples_per_pixel),
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_center = self.first_pixel_location
        + self.delta_u.scalar_mul(f64::from(i) + offset.x())
        + self.delta_v.scalar_mul(f64::from(j) + offset.y());
    let direction = pixel_center - self.camera_center;
    Ray::new(self.camera_center, direction)
    }

    fn sample_square() -> Vector3D {
       Vector3D::new(utils::random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
