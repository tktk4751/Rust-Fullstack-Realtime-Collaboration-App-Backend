use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;

use config::db::create_db_pool;

mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

pub mod schema;


fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = create_db_pool();

    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    println!("Starting server at: {}", &bind_address);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::auth_routes::configure)
            .configure(routes::invitation_routes::configure)
            .configure(routes::message_routes::configure)
            .configure(routes::project_routes::configure)
            .configure(routes::task_routes::configure)
    })
    .bind(&bind_address)?
    .run()
}
