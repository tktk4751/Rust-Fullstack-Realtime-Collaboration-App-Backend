use actix_web::{delete, get, patch, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::models::Project;
use crate::utils::db_operations;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectData {
    name: String,
    owner_id: i32,
}

#[post("/projects")]
async fn create_project(data: web::Json<CreateProjectData>) -> Result<HttpResponse> {
    let new_project = db_operations::create_project(&data).await?;
    Ok(HttpResponse::Created().json(new_project))
}

#[get("/projects/{project_id}")]
async fn get_project(project_id: web::Path<i32>) -> Result<HttpResponse> {
    let project = db_operations::get_project(project_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(project))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProjectData {
    name: Option<String>,
}

#[patch("/projects/{project_id}")]
async fn update_project(
    project_id: web::Path<i32>,
    data: web::Json<UpdateProjectData>,
) -> Result<HttpResponse> {
    let updated_project = db_operations::update_project(project_id.into_inner(), &data).await?;
    Ok(HttpResponse::Ok().json(updated_project))
}

#[delete("/projects/{project_id}")]
async fn delete_project(project_id: web::Path<i32>) -> Result<HttpResponse> {
    db_operations::delete_project(project_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_project);
    config.service(get_project);
    config.service(update_project);
    config.service(delete_project);
}
