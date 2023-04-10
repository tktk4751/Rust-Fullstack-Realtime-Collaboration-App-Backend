use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::invitations;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Invitation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "invitations"]
pub struct NewInvitation {
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: String,
}
