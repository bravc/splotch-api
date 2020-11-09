table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        content -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        pass_hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, users,);
