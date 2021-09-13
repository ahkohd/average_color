#[derive(Debug)]
pub enum ImageFormat {
    PNG,
    JPG,
    JPEG,
}

impl ImageFormat {
    pub fn from(ext: &str) -> Option<ImageFormat> {
        match ext {
            "png" => Some(ImageFormat::PNG),
            "jpg" => Some(ImageFormat::JPG),
            "jpeg" => Some(ImageFormat::JPEG),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Rgb {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Rgb {
    pub fn to_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}
