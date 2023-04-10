table! {
    pub users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
