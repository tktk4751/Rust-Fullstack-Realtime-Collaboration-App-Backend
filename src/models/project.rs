use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::projects;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub owner_id: Uuid,
}
