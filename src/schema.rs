table! {
    snippets (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        timestamp_start -> Int4,
        timestamp_end -> Int4,
        track_uri -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        pass_hash -> Varchar,
        spotify_refresh -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

joinable!(snippets -> users (user_id));

allow_tables_to_appear_in_same_query!(
    snippets,
    users,
);
