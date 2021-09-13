use tokio;

const TEST_IMAGES: [&str; 2] = ["test_images/1.png", "test_images/2.png"];

async fn average_of_multiple_images(test: fn(results: Vec<average_color::AverageColorResult>)) {
    let imgs = TEST_IMAGES.map(|img| img.into());
    test(average_color::get_averages_colors(&imgs).await)
}

async fn average_color(test: fn(result: average_color::AverageColorResult)) {
    test(average_color::get_average_color(&TEST_IMAGES[1].into()).await)
}

#[test]
fn get_average_colors() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = average_of_multiple_images(|results| {
        assert_eq!(results.len(), 2);
    });
    rt.block_on(future);
}

#[test]
fn get_average_color() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = average_color(|result| match result {
        Ok(color) => match color {
            Some(c) => assert_eq!(c.to_string(), String::from("rgb(160, 118, 22)")),
            None => {
                println!("[error]: Unable to extract color!");
                assert!(false)
            }
        },
        Err(err) => {
            println!("[error]: {}", err);
            assert!(false);
        }
    });
    rt.block_on(future);
}
