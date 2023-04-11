use chrono::{DateTime, NaiveDate, Utc};
use diesel::{Insertable, Queryable,AsChangeset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::tasks;
use crate::schema::tasks::dsl::tasks as tasks_table;

// 既存のタスクモデル
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
}


// データベース操作を追加
use diesel::prelude::*;
pub fn create_task(new_task: NewTask, connection: &mut PgConnection) -> QueryResult<Task> {
    diesel::insert_into(tasks_table)
        .values(new_task)
        .get_result(connection)
}

pub fn get_all_tasks(connection: &mut PgConnection) -> QueryResult<Vec<Task>> {
    tasks_table.load::<Task>(connection)
}

pub fn get_task_by_id(task_id: i32, connection: &mut PgConnection) -> QueryResult<Task> {
    tasks_table.find(task_id).get_result::<Task>(connection)
}

pub fn update_task(task_id: i32, updated_task: &NewTask, connection: &mut PgConnection) -> QueryResult<Task> {
    diesel::update(tasks_table.find(task_id))
        .set(updated_task)
        .get_result(connection)
}

pub fn delete_task(task_id: i32, connection: &mut PgConnection) -> QueryResult<usize> {
    diesel::delete(tasks_table.find(task_id))
        .execute(connection)
}