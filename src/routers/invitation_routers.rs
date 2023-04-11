use actix_web::web;
use crate::handlers::invitations::{create_invitation, delete_invitation, get_invitation, update_invitation,delete_invitation};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_invitation);
    config.service(get_invitation);
    config.service(update_invitation);
    config.service(delete_invitation);
}
