
#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    // Returns a string in the format "r g b\n"
    pub fn to_line(self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}