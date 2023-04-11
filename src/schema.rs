// @generated automatically by Diesel CLI.

diesel::table! {
    invitations (id) {
        id -> Uuid,
        project_id -> Uuid,
        sender_id -> Uuid,
        receiver_id -> Uuid,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        sender_id -> Int4,
        receiver_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        owner_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        project_id -> Int4,
        assignee_id -> Nullable<Uuid>,
        due_date -> Nullable<Date>,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        username -> Nullable<Varchar>,
    }
}

diesel::joinable!(projects -> users (owner_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> users (assignee_id));

diesel::allow_tables_to_appear_in_same_query!(
    invitations,
    messages,
    projects,
    tasks,
    users,
);
