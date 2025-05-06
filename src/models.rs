use crate::schema::{
    command_history, command_logs, command_stats, descriptions, interaction_logs, interaction_stats, rate_limits,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::command_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommandHistory {
    pub id: i32,
    pub command: String,
    pub arguments: Option<String>,
    pub user_id: i64,
    pub guild_id: Option<i64>,
    pub executed_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::command_history)]
pub struct NewCommandHistory {
    pub command: String,
    pub arguments: Option<String>,
    pub user_id: i64,
    pub guild_id: Option<i64>,
    pub executed_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::command_stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommandStat {
    pub id: i32,
    pub command: String,
    pub arguments: Option<String>,
    pub count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::command_stats)]
pub struct NewCommandStat {
    pub command: String,
    pub arguments: Option<String>,
    pub count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = command_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommandLog {
    pub id: i32,
    pub user_id: i64,
    pub command_name: String,
    pub executed_at: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = command_logs)]
pub struct NewCommandLog {
    pub user_id: i64,
    pub command_name: String,
    pub executed_at: i64,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::interaction_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InteractionLog {
    pub id: i32,
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub user_id: i64,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::interaction_logs)]
pub struct NewInteractionLog {
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub user_id: i64,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::interaction_stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InteractionStats {
    pub id: i32,
    pub interaction_type: String,
    pub count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::interaction_stats)]
pub struct NewInteractionStats {
    pub interaction_type: String,
    pub count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::interaction_stats)]
pub struct UpdateInteractionStats {
    pub count: i32,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::rate_limits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RateLimit {
    pub id: i32,
    pub user_id: i64,
    pub command: String,
    pub last_used: NaiveDateTime,
    pub count: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::rate_limits)]
pub struct NewRateLimit {
    pub user_id: i64,
    pub command: String,
    pub last_used: NaiveDateTime,
    pub count: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::rate_limits)]
pub struct UpdateRateLimit {
    pub last_used: NaiveDateTime,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::descriptions)]
pub struct Description {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub guild_id: i64,
    pub user_id: i64,
    pub timestamp: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_command_history_struct() {
        let now = Utc::now().naive_utc();
        let history = CommandHistory {
            id: 1,
            command: "test".to_string(),
            arguments: Some("arg1 arg2".to_string()),
            user_id: 123,
            guild_id: Some(456),
            executed_at: now,
        };
        assert_eq!(history.command, "test");
        assert_eq!(history.arguments, Some("arg1 arg2".to_string()));
        assert_eq!(history.user_id, 123);
        assert_eq!(history.guild_id, Some(456));
    }

    #[test]
    fn test_command_stat_struct() {
        let now = Utc::now().naive_utc();
        let stat = CommandStat {
            id: 1,
            command: "test".to_string(),
            arguments: Some("arg1 arg2".to_string()),
            count: 5,
            last_used: now,
        };
        assert_eq!(stat.command, "test");
        assert_eq!(stat.arguments, Some("arg1 arg2".to_string()));
        assert_eq!(stat.count, 5);
    }

    #[test]
    fn test_description_struct() {
        let now = Utc::now().naive_utc();
        let desc = Description {
            id: 1,
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            guild_id: 123,
            user_id: 456,
            timestamp: now,
        };
        assert_eq!(desc.key, "test_key");
        assert_eq!(desc.value, "test_value");
        assert_eq!(desc.guild_id, 123);
        assert_eq!(desc.user_id, 456);
    }

    #[test]
    fn test_interaction_log_struct() {
        let now = Utc::now().naive_utc();
        let log = InteractionLog {
            id: 1,
            interaction_type: "test_type".to_string(),
            interaction_id: "test_id".to_string(),
            guild_id: 123,
            user_id: 456,
            timestamp: now,
        };
        assert_eq!(log.interaction_type, "test_type");
        assert_eq!(log.interaction_id, "test_id");
        assert_eq!(log.guild_id, 123);
        assert_eq!(log.user_id, 456);
    }

    #[test]
    fn test_interaction_stat_struct() {
        let now = Utc::now().naive_utc();
        let stat = InteractionStats {
            id: 1,
            interaction_type: "test_type".to_string(),
            count: 5,
            last_used: now,
        };
        assert_eq!(stat.interaction_type, "test_type");
        assert_eq!(stat.count, 5);
    }

    #[test]
    fn test_rate_limit_struct() {
        let now = Utc::now().naive_utc();
        let limit = RateLimit {
            id: 1,
            user_id: 123,
            command: "test_command".to_string(),
            last_used: now,
            count: 5,
        };
        assert_eq!(limit.user_id, 123);
        assert_eq!(limit.command, "test_command");
        assert_eq!(limit.count, 5);
    }
}
