use chrono::{DateTime, NaiveDate, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub assignee_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub assignee_id: Option<i32>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
}
