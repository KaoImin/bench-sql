pub mod rpc;
pub mod db;
pub mod schema;
pub mod models;

use std::env;

#[tokio::main]
async fn main() {
    let url = env::args().nth(0).unwrap();

    println!("Hello, world!");
}
