#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn to_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_hex_code(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}
