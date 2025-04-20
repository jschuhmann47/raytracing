use crate::{color::Color, hittables::Hittables, ray::Ray, vector3d::Vector3D, ASPECT_RADIO};

pub struct PpmImage {
    width: u32,
    height: u32,
    max_color_value: u8,
    fields: Vec<Color>,
}

pub fn test_image(width: u32, camera_center: Vector3D, start: Vector3D, delta_u: Vector3D, delta_v: Vector3D, world: &Hittables) -> PpmImage {
    let height = generate_height(width);
    let mut fields: Vec<Color> = Vec::with_capacity(
        (width * height)
            .try_into()
            .expect("failed to parse width and height to usize"),
    );
    for j in 1..=height {
        for i in 1..=width {
            let pixel_center =
                start + (delta_u.scalar_mul(i.into())) + delta_v.scalar_mul(j.into());
            let direction = pixel_center - camera_center;
            let new_ray = Ray::new(camera_center, direction);
            let color = new_ray.color(world);
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

    pub fn max_color_value(&self) -> u8 {
        self.max_color_value
    }

    pub fn fields(&self) -> &Vec<Color> {
        &self.fields
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

pub fn generate_height(width: u32) -> u32 {
    let height: f64 = f64::from(width) / ASPECT_RADIO;
    let height = height.max(1.0);
    height.trunc() as u32
}
