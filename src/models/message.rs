use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::messages;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub content: &'a str,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
}
