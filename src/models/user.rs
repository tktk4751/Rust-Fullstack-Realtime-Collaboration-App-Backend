use diesel::prelude::*;
use chrono::{DateTime, Utc};
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
    pub username: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub fn create_user(connection: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(connection)
}

pub fn find_user_by_id(connection: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
    users::table
        .filter(users::id.eq(user_id))
        .first(connection)
}

pub fn find_user_by_email(connection: &mut PgConnection, user_email: &str) -> QueryResult<User> {
    users::table
        .filter(users::email.eq(user_email))
        .first(connection)
}
