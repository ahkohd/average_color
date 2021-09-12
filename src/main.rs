mod average_color;
mod enums;
mod utils;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let paths = &args[1..];
    let results = average_color::get_average_colors(&paths).await;

    println!("{:?}", results);
}
