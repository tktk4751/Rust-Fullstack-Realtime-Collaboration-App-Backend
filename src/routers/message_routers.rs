use actix_web::web;
use crate::handlers::messages::{create_message, delete_message, get_message, list_messages, update_message};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_message);
    config.service(get_message);
    config.service(update_message);
    config.service(delete_message);
    config.service(list_messages);
}
