#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Grayscale,
    Rgb,
    Rgba,
}

impl ColorType {
    pub fn channels(&self) -> u8 {
        match self {
            ColorType::Grayscale => 1,
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ColorType::Grayscale => 0,
            ColorType::Rgb => 1,
            ColorType::Rgba => 2,
        }
    }

    pub fn from_u8(value: u8) -> Option<ColorType> {
        match value {
            0 => Some(ColorType::Grayscale),
            1 => Some(ColorType::Rgb),
            2 => Some(ColorType::Rgba),
            _ => None,
        }
    }
}
