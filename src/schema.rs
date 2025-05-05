use diesel::table;

table! {
    descriptions (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    command_history (id) {
        id -> Int4,
        user -> Varchar,
        command -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    command_stats (id) {
        id -> Int4,
        command -> Varchar,
        arguments -> Text,
        count -> Int4,
        last_used -> Timestamp,
    }
}
