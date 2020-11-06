table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        pass_hash -> Varchar,
        created_at -> Timestamp,
    }
}
