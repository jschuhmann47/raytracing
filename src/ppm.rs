
use std::{fs::File, io::Write};

use crate::vector3d::Vector3D;

pub struct PpmImage {
    width: u32,
    height: u32,
    max_color_value: u8,
    fields: Vec<Vector3D>
}

pub fn test_image() -> PpmImage {
    PpmImage {
        width: 2,
        height: 1,
        max_color_value: 255,
        fields: vec![
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 1.0),
        ],
    }
}

impl ToString for PpmImage {
    fn to_string(&self) -> String {
        let mut text = String::from("P3\n");
        text.push_str(format!("{} {}\n", self.width, self.height).as_str());
        text.push_str(format!("{}\n", self.max_color_value).as_str());
        for (i, color) in self.fields.iter().enumerate() {
            println!("Generating line {} of {}", i+1, self.fields.len());
            text.push_str(color.to_u8_range().to_line().as_str()); 
        };
        text
    }
}

pub fn create_ppm_image(image: PpmImage, path: &str) {
    let mut file = File::create(path).expect("failed to create image");
    file.write_all(image.to_string().as_bytes()).expect("failed to write file")
}