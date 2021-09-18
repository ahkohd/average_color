pub mod enums;
pub mod utils;

use crate::enums::Rgb;
use async_std::path::Path;
use image::{DynamicImage, GenericImageView};

extern crate image;

pub type AverageColor = Option<Rgb>;
pub type AverageColorResult = Result<AverageColor, String>;

pub async fn get_averages_colors(paths: &[String]) -> Vec<AverageColorResult> {
    let mut results = vec![];

    let tasks = utils::join_parallel(paths.into_iter().map(|path| {
        async fn extract_average(path: String) -> AverageColorResult {
            get_average_color(&path).await
        }

        extract_average(path.to_string())
    }))
    .await;

    for task in tasks {
        results.push(task)
    }

    results
}

pub async fn get_average_color(path: &String) -> AverageColorResult {
    let file_exists = Path::new(&path).exists().await;

    if file_exists {
        let (img_type, ext) = utils::parse_path(&path);

        return match img_type {
            Some(_) => match image::open(&path) {
                Ok(img) => Ok(calculate_average(&img)),
                Err(err) => Err(format!("{:?}", err)),
            },
            None => Err(format!("Unsupported image type: {}", ext.unwrap_or(""))),
        };
    } else {
        return Err("File does not exists!".into());
    }
}

pub fn calculate_average(img: &DynamicImage) -> AverageColor {
    // See: https://stackoverflow.com/a/2541680/6784368

    let (width, height) = img.dimensions();

    let block_size = 5;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut rgb: [u32; 3] = [0, 0, 0];
    let mut count = 0;

    loop {
        let (x1, y1) = next_coordinates(width, x, y, block_size);

        if y1 > height - 1 {
            break;
        }

        let pixel = img.get_pixel(x1, y1);

        rgb[0] += pixel.0[0] as u32;
        rgb[1] += pixel.0[1] as u32;
        rgb[2] += pixel.0[2] as u32;

        count += 1;
        x = x1;
        y = y1;
    }

    rgb[0] = !!(rgb[0] / count);
    rgb[1] = !!(rgb[1] / count);
    rgb[2] = !!(rgb[2] / count);

    Some(Rgb {
        r: rgb[0] as u8,
        g: rgb[1] as u8,
        b: rgb[2] as u8,
    })
}

fn next_coordinates(width: u32, x: u32, y: u32, block_size: u32) -> (u32, u32) {
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
