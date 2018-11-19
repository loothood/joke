table! {
    active_user (id) {
        id -> Int4,
        user_name -> Varchar,
    }
}

table! {
    adjective (id) {
        id -> Int4,
        adjective_value -> Varchar,
    }
}

table! {
    user_adjective (id) {
        id -> Int4,
        user_id -> Int4,
        adjective_id -> Int4,
        count -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    active_user,
    adjective,
    user_adjective,
);
