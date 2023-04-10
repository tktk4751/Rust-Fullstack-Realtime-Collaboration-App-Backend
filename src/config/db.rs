use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("postgresql://postgres:FL5x3Iz8RMw4kbsE5v5x@containers-us-west-14.railway.app:6465/railway").expect("postgresql://postgres:FL5x3Iz8RMw4kbsE5v5x@containers-us-west-14.railway.app:6465/railway");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
