use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(api::handlers::get_users))
            .route("/users/{id}", web::get().to(api::handlers::get_user_by_id))
            .route("/users", web::post().to(api::handlers::add_user))
            .route("/users/{id}", web::delete().to(api::handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}