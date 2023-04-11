use actix_web::{delete, get, patch, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use crate::models::Invitation;
use crate::models::invitation::{create_invitation, get_invitation, update_invitation, delete_invitation};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateInvitationData {
    project_id: Uuid,
    sender_id: Uuid,
    receiver_id: Uuid,
}

#[post("/invitations")]
async fn create_invitation(data: web::Json<CreateInvitationData>) -> Result<HttpResponse> {
    let new_invitation = db_operations::create_invitation(&data).await?;
    Ok(HttpResponse::Created().json(new_invitation))
}

#[get("/invitations/{invitation_id}")]
async fn get_invitation(invitation_id: web::Path<Uuid>) -> Result<HttpResponse> {
    let invitation = db_operations::get_invitation(invitation_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(invitation))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateInvitationData {
    status: String,
}

#[patch("/invitations/{invitation_id}")]
async fn update_invitation(
    invitation_id: web::Path<Uuid>,
    data: web::Json<UpdateInvitationData>,
) -> Result<HttpResponse> {
    let updated_invitation = db_operations::update_invitation(invitation_id.into_inner(), &data).await?;
    Ok(HttpResponse::Ok().json(updated_invitation))
}

#[delete("/invitations/{invitation_id}")]
async fn delete_invitation(invitation_id: web::Path<Uuid>) -> Result<HttpResponse> {
    db_operations::delete_invitation(invitation_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_invitation);
    config.service(get_invitation);
    config.service(update_invitation);
    config.service(delete_invitation);
}
