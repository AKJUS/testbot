use diesel::table;
use diesel::allow_tables_to_appear_in_same_query;

table! {
    descriptions (id) {
        id -> Int8,
        key -> Varchar,
        value -> Varchar,
        guild_id -> Int8,
        user_id -> Int8,
        timestamp -> Timestamp,
    }
}

table! {
    command_history (id) {
        id -> Int4,
        command -> Varchar,
        arguments -> Nullable<Varchar>,
        user_id -> Int8,
        guild_id -> Nullable<Int8>,
        executed_at -> Timestamp,
    }
}

table! {
    command_logs (id) {
        id -> Int4,
        user_id -> Int8,
        command_name -> Text,
        executed_at -> Timestamp,
    }
}

table! {
    command_stats (id) {
        id -> Int4,
        command -> Varchar,
        arguments -> Nullable<Varchar>,
        count -> Int4,
        last_used -> Timestamp,
    }
}

table! {
    interaction_logs (id) {
        id -> Int4,
        interaction_type -> Varchar,
        interaction_id -> Varchar,
        guild_id -> Int8,
        user_id -> Int8,
        timestamp -> Timestamp,
    }
}

table! {
    interaction_stats (id) {
        id -> Int4,
        interaction_type -> Varchar,
        count -> Int4,
        last_used -> Timestamp,
    }
}

table! {
    rate_limits (id) {
        id -> Int4,
        user_id -> Int8,
        command -> Varchar,
        last_used -> Timestamp,
        count -> Int4,
    }
}

diesel::joinable!(command_history -> descriptions (guild_id));
diesel::joinable!(interaction_logs -> descriptions (guild_id));

allow_tables_to_appear_in_same_query!(
    command_history,
    command_stats,
    descriptions,
    interaction_logs,
    interaction_stats,
    rate_limits,
);
