use average_color::{
    enums::{ImageFormat, Rgb},
    AverageColorResult,
};
use std::env;
use tokio;

const TEST_IMAGES: [&str; 2] = ["./test_images/1.png", "test_images/2.png"];

fn abs_path(path: &str) -> String {
    format!(
        "{}/tests/{}",
        str::replace(env::current_dir().unwrap().to_str().unwrap(), "\"", ""),
        path
    )
}

async fn average_of_multiple_images(test: fn(results: Vec<average_color::AverageColorResult>)) {
    let imgs = TEST_IMAGES.map(|img| abs_path(img));
    test(average_color::get_averages_colors(&imgs).await)
}

async fn average_of_an_image(test: fn(result: average_color::AverageColorResult)) {
    test(average_color::get_average_color(&abs_path(TEST_IMAGES[1])).await)
}

fn test_average_color_result(result: &AverageColorResult, expected: &Rgb) {
    match result {
        Ok(color) => match color {
            Some(color) => assert_eq!(color, expected),
            None => {
                println!("[error]: Unable to extract color!");
                assert!(false)
            }
        },
        Err(err) => {
            println!("[error]: {}", err);
            assert!(false);
        }
    }
}

#[test]
fn get_average_colors() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = average_of_multiple_images(|results| {
        assert_eq!(results.len(), 2);

        let expected_average_colors = [
            Rgb {
                r: 153,
                g: 138,
                b: 123,
            },
            Rgb {
                r: 160,
                g: 118,
                b: 22,
            },
        ];

        for (index, result) in results.iter().enumerate() {
            test_average_color_result(result, &expected_average_colors[index])
        }
    });
    rt.block_on(future);
}

#[test]
fn get_average_color() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = average_of_an_image(|result| {
        test_average_color_result(
            &result,
            &Rgb {
                r: 160,
                g: 118,
                b: 22,
            },
        )
    });
    rt.block_on(future);
}

#[test]
fn get_extension() {
    assert_eq!(
        average_color::utils::get_extension("test.png").unwrap(),
        "png"
    )
}

#[test]
fn parse_path() {
    assert_eq!(
        average_color::utils::parse_path("test.png"),
        (Some(ImageFormat::PNG), Some("png"))
    )
}

#[test]
fn get_hex_code() {
    assert_eq!(
        Rgb {
            r: 160,
            g: 118,
            b: 22,
        }
        .to_hex_code(),
        "#a07616"
    )
}
