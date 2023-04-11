use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;

use config::db::create_db_pool;
use middleware::auth_middleware::Authentication;

mod config;
mod handlers;
mod middleware;
mod models;
mod routers; // ここをroutesからroutersに変更
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
            .wrap(Authentication) // Authenticationミドルウェアを追加
            .configure(routers::auth::init_routes)
            .configure(routers::invitation::init_routes)
            .configure(routers::message::init_routes) // ここをroutesからroutersに変更
            .configure(routers::project::init_routes)
            .configure(routers::task::init_routes)
    })
    .bind(&bind_address)?
    .run()
}
