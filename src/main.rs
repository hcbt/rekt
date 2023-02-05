use log::{info};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api_error;
mod api;
mod db;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Load variables
    let host = env::var("API_HOST").expect("Host not set");
    let port = env::var("API_PORT").expect("Port not set");

    // Initialize database connection
    db::init();
    
    HttpServer::new(move || {
        App::new()
            .configure(api::handlers::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}