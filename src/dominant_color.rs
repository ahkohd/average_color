use crate::enums::Rgb;
use crate::utils;
use async_std::path::Path;
use image::{DynamicImage, GenericImageView};

extern crate image;

type DominantColor = Option<Rgb>;
type DominantColorResult = Result<DominantColor, String>;

pub async fn get_dominant_colors(paths: &[String]) -> Vec<DominantColorResult> {
    let mut colors = vec![];

    let tasks = utils::join_parallel(
        paths
            .into_iter()
            .map(|path| get_dominant_color(path.to_string())),
    )
    .await;

    for task in tasks {
        colors.push(task)
    }

    colors
}

pub async fn get_dominant_color(path: String) -> DominantColorResult {
    let file_exists = Path::new(&path).exists().await;

    if file_exists {
        let (img_type, ext) = utils::parse_path(&path);

        return match img_type {
            Some(_) => match image::open(&path) {
                Ok(img) => Ok(calculate_dominant_color(&img)),
                Err(err) => Err(format!("{:?}", err)),
            },
            None => Err(format!("Unsupported image type: {}", ext.unwrap_or(""))),
        };
    } else {
        return Err("File does not exists!".into());
    }
}

fn calculate_dominant_color(img: &DynamicImage) -> DominantColor {
    // See: https://stackoverflow.com/a/2541680/6784368

    let (width, height) = img.dimensions();

    let block_size = 5;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut rgb: Rgb = Rgb { r: 0, g: 0, b: 0 };
    let mut count = 0;

    loop {
        let (x1, y1) = get_next_coordinates(width, x, y, block_size);

        if y1 > height - 1 {
            break;
        }

        let pixel = img.get_pixel(x1, y1);

        rgb.r += pixel.0[0] as u32;
        rgb.g += pixel.0[1] as u32;
        rgb.b += pixel.0[2] as u32;

        count += 1;
        x = x1;
        y = y1;
    }

    rgb.r = !!(rgb.r / count);
    rgb.g = !!(rgb.g / count);
    rgb.b = !!(rgb.b / count);

    Some(rgb)
}

fn get_next_coordinates(width: u32, x: u32, y: u32, block_size: u32) -> (u32, u32) {
    let mut next_x = x;
    let mut next_y = y;
    let w = width - 1;

    if x < w && x + block_size < w {
        next_x += block_size;
    } else {
        next_x = 0;
        next_y = y + block_size;
    }

    (next_x, next_y)
}
