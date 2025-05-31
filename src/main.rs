use camera::Camera;
use hittables::Hittables;
use sphere::Sphere;
use std::{fs::File, io::Write};
use vector3d::Vector3D;
use viewport::Viewport;

mod camera;
mod color;
mod hittable;
mod hittables;
mod interval;
mod ppm;
mod ray;
mod sphere;
mod utils;
mod vector3d;
mod viewport;

const ASPECT_RADIO: f64 = 16.0 / 9.0;

fn main() {
    let width = 400;

    let mut world = Hittables::new();
    world.add(Box::new(Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3D::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = Camera::initialize(ASPECT_RADIO, width, 100);

    let image = camera.render(world);

    create_image(image, "image.ppm");
}

pub fn create_image(image: ppm::PpmImage, path: &str) {
    let mut file = File::create(path).expect("failed to create image");
    file.write_all(image.to_string().as_bytes())
        .expect("failed to write file")
}
