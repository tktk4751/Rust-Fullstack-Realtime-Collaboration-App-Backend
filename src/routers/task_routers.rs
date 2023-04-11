use actix_web::web;
use crate::handlers::tasks::{create_task, delete_task, get_task, list_tasks, update_task};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_task);
    config.service(get_task);
    config.service(update_task);
    config.service(delete_task);
    config.service(list_tasks);
}
