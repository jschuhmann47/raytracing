use crate::color::Color;

pub struct PpmImage {
    width: u32,
    height: u32,
    max_color_value: u8,
    fields: Vec<Color>,
}

impl PpmImage {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn max_color_value(&self) -> u8 {
        self.max_color_value
    }

    pub fn fields(&self) -> &Vec<Color> {
        &self.fields
    }

    pub fn new(width: u32, height: u32, max_color_value: u8, fields: Vec<Color>) -> Self {
        Self {
            width,
            height,
            max_color_value,
            fields,
        }
    }
}

impl ToString for PpmImage {
    fn to_string(&self) -> String {
        let mut text = String::from("P3\n");
        text.push_str(format!("{} {}\n", self.width(), self.height()).as_str());
        text.push_str(format!("{}\n", self.max_color_value()).as_str());
        for (i, color) in self.fields().iter().enumerate() {
            println!("Generating: {}%", ((i + 1) * 100 / self.fields.len()));
            text.push_str(color.to_line().as_str());
        }
        text
    }
}
