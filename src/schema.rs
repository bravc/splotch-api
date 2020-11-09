table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        pass_hash -> Varchar,
        created_at -> Timestamp,
    }
}
