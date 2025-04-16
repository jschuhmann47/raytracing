use vector3d::Vector3D;
use viewport::Viewport;

mod ppm;
mod ray;
mod vector3d;
mod viewport;

fn main() {
    let focal_length = 1.0;
    let camera_center = Vector3D::new(0.0, 0.0, 0.0);
    let viewport = Viewport::new(2.0 * 9.0 / 16.0, 2.0);
    let image = ppm::test_image(400);

    let delta_u = viewport
        .viewpoint_u()
        .scalar_mul(1.0 / f64::from(image.width()));
    let delta_v = viewport
        .viewpoint_v()
        .scalar_mul(1.0 / f64::from(image.height()));

    let viewport_upper_left = camera_center
        - Vector3D::new(0.0, 0.0, focal_length)
        - viewport.viewpoint_u().scalar_mul(0.5)
        - viewport.viewpoint_v().scalar_mul(0.5);

    let pixel00_location = viewport_upper_left + (delta_u + delta_v).scalar_mul(0.5);

    let image = ppm::test_image(width)

    ppm::create_ppm_image(image, "image.ppm");
}
