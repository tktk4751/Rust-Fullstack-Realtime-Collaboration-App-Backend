use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::messages;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub content: String,
    pub sender_id: i32,
    pub receiver_id: i32,
}
