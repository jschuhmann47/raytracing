use std::{fs::File, io::Write};
use vector3d::Vector3D;
use viewport::Viewport;

mod color;
mod ppm;
mod ray;
mod vector3d;
mod viewport;
mod sphere;

const ASPECT_RADIO: f64 = 16.0 / 9.0;

fn main() {
    let focal_length = 1.0;
    let camera_center = Vector3D::new(0.0, 0.0, 0.0);
    let width = 400;
    let height: f64 = f64::from(width) / ASPECT_RADIO;
    let viewport = Viewport::new(2.0 * f64::from(width) / height, 2.0);

    let delta_u = viewport.viewpoint_u().scalar_div(f64::from(width));
    let delta_v = viewport.viewpoint_v().scalar_div(height);

    let viewport_upper_left = camera_center
        - Vector3D::new(0.0, 0.0, focal_length)
        - viewport.viewpoint_u().scalar_div(2.0)
        - viewport.viewpoint_v().scalar_div(2.0);

    let pixel00_location = viewport_upper_left + (delta_u + delta_v).scalar_div(2.0);

    let image = ppm::test_image(width, camera_center, pixel00_location, delta_u, delta_v);

    create_image(image, "image.ppm");
}

pub fn create_image(image: ppm::PpmImage, path: &str) {
    let mut file = File::create(path).expect("failed to create image");
    file.write_all(image.to_string().as_bytes())
        .expect("failed to write file")
}
