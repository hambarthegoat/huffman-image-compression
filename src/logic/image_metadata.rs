use super::color_type::ColorType;

#[derive(Debug)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
}
