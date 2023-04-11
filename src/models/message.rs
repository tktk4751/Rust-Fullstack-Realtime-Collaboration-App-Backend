use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::QueryResult;


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

// メッセージを作成する
pub fn create_message(new_message: NewMessage, connection: &mut PgConnection) -> QueryResult<Message> {
    diesel::insert_into(messages::table)
        .values(new_message)
        .get_result(connection)
}

// すべてのメッセージを取得する
pub fn get_all_messages(connection: &mut PgConnection) -> QueryResult<Vec<Message>> {
    messages::table.load::<Message>(connection)
}

// 特定のメッセージを取得する
pub fn get_message_by_id(message_id: i32, connection: &mut PgConnection) -> QueryResult<Message> {
    messages::table.find(message_id).get_result::<Message>(connection)
}

// メッセージを削除する
pub fn delete_message(message_id: i32, connection: &mut PgConnection) -> QueryResult<usize> {
    diesel::delete(messages::table.find(message_id)).execute(connection)
}
