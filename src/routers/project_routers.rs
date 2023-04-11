use actix_web::web;
use crate::handlers::projects::{create_project, delete_project, get_project, list_projects, update_project};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_project);
    config.service(get_project);
    config.service(update_project);
    config.service(delete_project);
    config.service(list_projects);
}
