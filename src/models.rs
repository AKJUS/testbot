use crate::schema::descriptions;
use diesel::{AsChangeset, Insertable, Queryable};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::{command_logs, command_stats, interaction_logs, interaction_stats, rate_limits};
use serde::{Serialize, Deserialize};

#[derive(Queryable, AsChangeset)]
pub struct Description {
    pub key: String,
    pub value: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = descriptions)]
pub struct NewDescription<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::command_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommandHistory {
    pub id: i64,
    pub command: String,
    pub arguments: String,
    pub user_id: i64,
    pub executed_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::command_history)]
pub struct NewCommandHistory {
    pub command: String,
    pub arguments: String,
    pub user_id: i64,
    pub executed_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::command_stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommandStats {
    pub id: i64,
    pub command: String,
    pub arguments: String,
    pub count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::command_stats)]
pub struct NewCommandStats {
    pub command: String,
    pub arguments: String,
    pub count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::command_stats)]
pub struct UpdateCommandStats {
    pub count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct CommandLog {
    pub id: i32,
    pub user_id: i64,
    pub command_name: String,
    pub executed_at: i64,
}

#[derive(Insertable)]
#[diesel(table_name = command_logs)]
pub struct NewCommandLog {
    pub user_id: i64,
    pub command_name: String,
    pub executed_at: i64,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::interaction_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InteractionLog {
    pub id: i64,
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub user_id: i64,
    pub executed_at: NaiveDateTime,
    pub duration: f64,
    pub success: bool,
    pub error_type: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::interaction_logs)]
pub struct NewInteractionLog {
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub user_id: i64,
    pub executed_at: NaiveDateTime,
    pub duration: f64,
    pub success: bool,
    pub error_type: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::interaction_stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InteractionStats {
    pub id: i64,
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub count: i64,
    pub total_duration: f64,
    pub failure_count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::interaction_stats)]
pub struct NewInteractionStats {
    pub interaction_type: String,
    pub interaction_id: String,
    pub guild_id: i64,
    pub count: i64,
    pub total_duration: f64,
    pub failure_count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::interaction_stats)]
pub struct UpdateInteractionStats {
    pub count: i64,
    pub total_duration: f64,
    pub failure_count: i64,
    pub last_used: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::rate_limits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RateLimit {
    pub id: i64,
    pub interaction_type: String,
    pub guild_id: i64,
    pub hits: i64,
    pub reset_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::rate_limits)]
pub struct NewRateLimit {
    pub interaction_type: String,
    pub guild_id: i64,
    pub hits: i64,
    pub reset_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::rate_limits)]
pub struct UpdateRateLimit {
    pub hits: i64,
    pub reset_at: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_struct() {
        let desc = Description {
            key: "foo".to_string(),
            value: "bar".to_string(),
        };
        assert_eq!(desc.key, "foo");
        assert_eq!(desc.value, "bar");
    }

    #[test]
    fn test_new_description_struct() {
        let new_desc = NewDescription {
            key: "foo",
            value: "bar",
        };
        assert_eq!(new_desc.key, "foo");
        assert_eq!(new_desc.value, "bar");
    }

    #[test]
    fn test_command_log() {
        let log = CommandLog {
            id: 1,
            user_id: 123,
            command_name: "test".to_string(),
            executed_at: 1234567890,
        };
        assert_eq!(log.id, 1);
        assert_eq!(log.user_id, 123);
        assert_eq!(log.command_name, "test");
        assert_eq!(log.executed_at, 1234567890);
    }

    #[test]
    fn test_command_stats() {
        let now = chrono::Utc::now().naive_utc();
        let stats = CommandStats {
            id: 1,
            command: "test".to_string(),
            arguments: "arg1 arg2".to_string(),
            count: 42,
            last_used: now,
        };
        assert_eq!(stats.id, 1);
        assert_eq!(stats.command, "test");
        assert_eq!(stats.arguments, "arg1 arg2");
        assert_eq!(stats.count, 42);
        assert_eq!(stats.last_used, now);
    }

    #[test]
    fn test_interaction_log() {
        let now = chrono::Utc::now().naive_utc();
        let log = InteractionLog {
            id: 1,
            interaction_type: "slash_command".to_string(),
            interaction_id: "test_command".to_string(),
            guild_id: 123,
            user_id: 456,
            executed_at: now,
            duration: 0.5,
            success: true,
            error_type: None,
        };
        assert_eq!(log.id, 1);
        assert_eq!(log.interaction_type, "slash_command");
        assert_eq!(log.interaction_id, "test_command");
        assert_eq!(log.guild_id, 123);
        assert_eq!(log.user_id, 456);
        assert_eq!(log.duration, 0.5);
        assert!(log.success);
        assert!(log.error_type.is_none());
    }

    #[test]
    fn test_interaction_stats() {
        let now = chrono::Utc::now().naive_utc();
        let stats = InteractionStats {
            id: 1,
            interaction_type: "slash_command".to_string(),
            interaction_id: "test_command".to_string(),
            guild_id: 123,
            count: 42,
            total_duration: 21.0,
            failure_count: 2,
            last_used: now,
        };
        assert_eq!(stats.id, 1);
        assert_eq!(stats.interaction_type, "slash_command");
        assert_eq!(stats.interaction_id, "test_command");
        assert_eq!(stats.guild_id, 123);
        assert_eq!(stats.count, 42);
        assert_eq!(stats.total_duration, 21.0);
        assert_eq!(stats.failure_count, 2);
        assert_eq!(stats.last_used, now);
    }

    #[test]
    fn test_rate_limit() {
        let now = chrono::Utc::now().naive_utc();
        let limit = RateLimit {
            id: 1,
            interaction_type: "slash_command".to_string(),
            guild_id: 123,
            hits: 5,
            reset_at: now,
        };
        assert_eq!(limit.id, 1);
        assert_eq!(limit.interaction_type, "slash_command");
        assert_eq!(limit.guild_id, 123);
        assert_eq!(limit.hits, 5);
        assert_eq!(limit.reset_at, now);
    }
}
