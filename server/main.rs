#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(pool.clone())
            .route("/todos", web::get().to(handlers::get_todos))
            .route("/todos/{id}", web::get().to(handlers::get_todo_by_id))
            .route("/todos", web::post().to(handlers::add_todo))
            .route("/todos/{id}", web::delete().to(handlers::delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
