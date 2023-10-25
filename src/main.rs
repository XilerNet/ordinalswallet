#![feature(exclusive_range_pattern)]

pub mod db;
pub mod domain_length;
pub mod model;
pub mod utils;

#[tokio::main]
async fn main() {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    println!("Hello, world!");
}
