use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::tasks;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub creator_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDateTime>,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub project_id: Uuid,
    pub creator_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDateTime>,
}
