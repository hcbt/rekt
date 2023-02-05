use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_identity::IdentityMiddleware;
use actix_web::{App, HttpServer, cookie::Key};
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
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .build()
            )
            .configure(api::handlers::users::users_routes)
            .configure(api::handlers::auth::auth_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}