use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
