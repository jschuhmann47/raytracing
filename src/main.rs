use std::{fs::File, io::Write};

fn main() {
    let image = PpmImage{
        width: 2,
        height: 1,
        max_color_value: 255,
        fields: vec![Color{red: 255,green: 0,blue: 0},Color{red: 255,green: 0,blue: 255}]
    };
    create_ppm_image(image);
}

struct PpmImage {
    width: u32,
    height: u32,
    max_color_value: u8,
    fields: Vec<Color>
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl ToString for PpmImage {
    fn to_string(&self) -> String {
        let mut text = String::from("P3\n");
        text.push_str(format!("{} {}\n", self.width, self.height).as_str());
        text.push_str(format!("{}\n", self.max_color_value).as_str());
        for color in &self.fields {
            text.push_str(format!("{} {} {}\n", color.red, color.green, color.blue).as_str()); 
        };
        text
    }
}

fn create_ppm_image(image: PpmImage) {
    let mut file = File::create("image.ppm").expect("failed to create image");
    file.write_all(image.to_string().as_bytes()).expect("failed to write file")
}