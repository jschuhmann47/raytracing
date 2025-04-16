use std::{fs::File, io::Write};

use crate::{ray::Ray, vector3d::Vector3D};

pub struct PpmImage {
    width: u32,
    height: u32,
    max_color_value: u8,
    fields: Vec<Vector3D>,
}

pub fn test_image(width: u32, start: Vector3D, delta_u: Vector3D, delta_v: Vector3D) -> PpmImage {
    let height = generate_height(width);
    let mut fields: Vec<Vector3D> = Vec::with_capacity((width*height).try_into().expect("failed to alloc for fields"));
    for i in 0..width {
        for j in 0..width {
            let pixel_center = start + (delta_u.scalar_mul(i.into())) + delta_v.scalar_mul(j.into());
            let direction = pixel_center - start;
            let new_ray = Ray::new(start, direction);
            let color = blend_color(0.4, Vector3D::new(1.0, 1.0, 1.0).to_u8_range(), Vector3D::new(0.5, 0.7, 1.0).to_u8_range());
            fields.push(color);
        }
    }
    PpmImage {
        width,
        height,
        max_color_value: 255,
        fields,
    }
}

impl PpmImage {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl ToString for PpmImage {
    fn to_string(&self) -> String {
        let mut text = String::from("P3\n");
        text.push_str(format!("{} {}\n", self.width, self.height).as_str());
        text.push_str(format!("{}\n", self.max_color_value).as_str());
        for (i, color) in self.fields.iter().enumerate() {
            println!("Generating line {} of {}", i + 1, self.fields.len());
            text.push_str(color.to_u8_range().to_line().as_str());
        }
        text
    }
}

pub fn create_ppm_image(image: PpmImage, path: &str) {
    let mut file = File::create(path).expect("failed to create image");
    file.write_all(image.to_string().as_bytes())
        .expect("failed to write file")
}

pub fn generate_height(width: u32) -> u32 {
    let height = width / (16/9);
    height.min(1)
} 

// alpha goes from zero to one
fn blend_color(alpha: f64, start: Vector3D, end: Vector3D) -> Vector3D {
    start.scalar_mul(1.0-alpha) + end.scalar_mul(alpha)
}