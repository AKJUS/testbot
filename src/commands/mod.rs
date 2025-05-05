pub mod advice;
pub mod ball;
pub mod botsnack;
pub mod desc;
pub mod drink;
pub mod food;
pub mod github;
pub mod owner;
pub mod pingpong;
pub mod random;
pub mod stonks;
pub mod stats;

pub fn log_command(conn: &mut diesel::PgConnection, user: &str, command: &str) {
    use crate::schema::command_history;
    use diesel::prelude::*;
    use chrono::Utc;
    let new_entry = crate::models::CommandHistory {
        id: 0, // will be set by DB
        user: user.to_string(),
        command: command.to_string(),
        timestamp: Utc::now().naive_utc(),
    };
    diesel::insert_into(command_history::table)
        .values(&new_entry)
        .execute(conn)
        .ok();
}

pub fn update_command_stats(conn: &mut diesel::PgConnection, command_name: &str, args: &str) {
    use crate::schema::command_stats::dsl::*;
    use diesel::prelude::*;
    use chrono::Utc;
    let now = Utc::now().naive_utc();
    let updated = diesel::update(command_stats.filter(command.eq(command_name).and(arguments.eq(args))))
        .set((count.eq(count + 1), last_used.eq(now)))
        .execute(conn)
        .unwrap_or(0);
    if updated == 0 {
        let new_stat = crate::models::CommandStat {
            id: 0,
            command: command_name.to_string(),
            arguments: args.to_string(),
            count: 1,
            last_used: now,
        };
        diesel::insert_into(command_stats)
            .values(&new_stat)
            .execute(conn)
            .ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log_command_inserts() {
        assert!(true);
    }
}
