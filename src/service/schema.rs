table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        checked -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    tasks,
);
