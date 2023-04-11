use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::QueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::invitations;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Invitation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "invitations"]
pub struct NewInvitation {
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: String,
}

// 招待を作成する
pub fn create_invitation(new_invitation: NewInvitation, connection: &mut PgConnection) -> QueryResult<Invitation> {
    diesel::insert_into(invitations::table)
        .values(new_invitation)
        .get_result(connection)
}

// すべての招待を取得する
pub fn get_all_invitations(connection: &mut PgConnection) -> QueryResult<Vec<Invitation>> {
    invitations::table.load::<Invitation>(connection)
}

// 特定の招待を取得する
pub fn get_invitation_by_id(invitation_id: Uuid, connection: &mut PgConnection) -> QueryResult<Invitation> {
    invitations::table.find(invitation_id).get_result::<Invitation>(connection)
}

// 招待を更新する
pub fn update_invitation_status(invitation_id: Uuid, new_status: &str, connection: &mut PgConnection) -> QueryResult<Invitation> {
    diesel::update(invitations::table.find(invitation_id))
        .set(invitations::status.eq(new_status))
        .get_result(connection)
}

// 招待を削除する
pub fn delete_invitation(invitation_id: Uuid, connection: &mut PgConnection) -> QueryResult<usize> {
    diesel::delete(invitations::table.find(invitation_id)).execute(connection)
}
