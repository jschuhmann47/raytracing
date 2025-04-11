mod ppm;
mod vector3d;
mod ray;

fn main() {
    let image = ppm::test_image();
    ppm::create_ppm_image(image, "image.ppm");
}
