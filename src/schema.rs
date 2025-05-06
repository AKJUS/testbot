use diesel::table;

table! {
    descriptions (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    command_history (id) {
        id -> Int8,
        command -> Text,
        arguments -> Text,
        user_id -> Int8,
        executed_at -> Timestamp,
    }
}

table! {
    command_logs (id) {
        id -> Int4,
        user_id -> Int8,
        command_name -> Text,
        executed_at -> Int8,
    }
}

table! {
    command_stats (id) {
        id -> Int8,
        command -> Text,
        arguments -> Text,
        count -> Int8,
        last_used -> Timestamp,
    }
}

table! {
    interaction_logs (id) {
        id -> Int8,
        interaction_type -> Text,
        interaction_id -> Text,
        guild_id -> Int8,
        user_id -> Int8,
        executed_at -> Timestamp,
        duration -> Float8,
        success -> Bool,
        error_type -> Nullable<Text>,
    }
}

table! {
    interaction_stats (id) {
        id -> Int8,
        interaction_type -> Text,
        interaction_id -> Text,
        guild_id -> Int8,
        count -> Int8,
        total_duration -> Float8,
        failure_count -> Int8,
        last_used -> Timestamp,
    }
}

table! {
    rate_limits (id) {
        id -> Int8,
        interaction_type -> Text,
        guild_id -> Int8,
        hits -> Int8,
        reset_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    command_history,
    command_stats,
    interaction_logs,
    interaction_stats,
    rate_limits,
);
