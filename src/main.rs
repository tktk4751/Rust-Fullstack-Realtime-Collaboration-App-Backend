use config::db::create_db_pool;
use actix_web::{middleware, web, App, HttpServer};

mod config;
mod controllers;
mod models;
mod routes;
mod utils;
pub mod schema;


fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = create_db_pool();

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    println!("Starting server at: {}", &bind_address);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::configure)
            .configure(auth_routes::auth_routes)
            .configure(task_routes::task_routes)
    })
    .bind(&bind_address)?
    .run()
}
