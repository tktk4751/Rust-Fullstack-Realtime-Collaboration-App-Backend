use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::models::task::{NewTask, Task};

#[derive(Deserialize)]
pub struct CreateTaskData {
    title: String,
    description: Option<String>,
    project_id: i32,
    assignee_id: Option<i32>,
    due_date: Option<String>,
    status: String,
}

pub async fn create_task(task_data: web::Json<CreateTaskData>) -> impl Responder {
    let new_task = NewTask {
        title: task_data.title.clone(),
        description: task_data.description.clone(),
        project_id: task_data.project_id,
        assignee_id: task_data.assignee_id,
        due_date: task_data.due_date.clone(),
        status: task_data.status.clone(),
    };

    // TODO: Save the new_task to the database

    HttpResponse::Created().finish()
}

pub async fn get_task(task_id: web::Path<i32>) -> impl Responder {
    // TODO: Fetch the task from the database

    HttpResponse::Ok().json(Task {
        // Use dummy data for now
        id: *task_id,
        title: "Example Task".to_string(),
        description: Some("An example task description.".to_string()),
        project_id: 1,
        assignee_id: Some(1),
        due_date: None,
        status: "In Progress".to_string(),
    })
}

#[derive(Deserialize)]
pub struct UpdateTaskData {
    title: Option<String>,
    description: Option<String>,
    assignee_id: Option<i32>,
    due_date: Option<String>,
    status: Option<String>,
}

pub async fn update_task(task_id: web::Path<i32>, task_data: web::Json<UpdateTaskData>) -> impl Responder {
    // TODO: Update the task in the database

    HttpResponse::Ok().finish()
}

pub async fn delete_task(task_id: web::Path<i32>) -> impl Responder {
    // TODO: Delete the task from the database

    HttpResponse::NoContent().finish()
}
