use crate::db::DbPool;
use crate::db::Pool;
use crate::metrics;
use crate::models::{CommandHistory, CommandStat};
use crate::schema::{command_history, command_stats};
use crate::utils::time::get_current_time;
use crate::Data;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use mockall::mock;
use mockall::predicate::*;
use poise::serenity_prelude::{Context, User};
use poise::serenity_prelude::{User, UserId};
use std::error::Error;
use std::time::{Duration, Instant};

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
pub mod stats;
pub mod stonks;

// Re-export commonly used items
pub use self::{
    advice::advice,
    ball::ball,
    botsnack::botsnack,
    desc::set,
    drink::drink,
    food::food,
    github::github,
    owner::quit,
    pingpong::ping,
    random::random,
    stats::stats,
    stonks::{graph, stonkcomp, stonks},
};

/// Command context with timing information
pub struct CommandContext {
    pub command: String,
    pub args: Vec<String>,
    pub start_time: Instant,
}

impl CommandContext {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            start_time: Instant::now(),
        }
    }

    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Log a command execution to the database
pub async fn log_command(
    pool: &DbPool,
    command: &str,
    args: &[String],
    user: &User,
) -> Result<(), Box<dyn Error>> {
    use crate::schema::command_history;
    let conn = &mut pool.get()?;

    diesel::insert_into(command_history::table)
        .values((
            command_history::command.eq(command),
            command_history::arguments.eq(args.join(" ")),
            command_history::user_id.eq(user.id.get() as i64),
            command_history::executed_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)?;

    Ok(())
}

/// Update command statistics in the database
pub async fn update_command_stats(
    pool: &DbPool,
    command: &str,
    args: &[String],
) -> Result<(), Box<dyn Error>> {
    use crate::schema::command_stats;
    let conn = &mut pool.get()?;
    let now = chrono::Utc::now().naive_utc();

    diesel::insert_into(command_stats::table)
        .values((
            command_stats::command.eq(command),
            command_stats::arguments.eq(args.join(" ")),
            command_stats::count.eq(1),
            command_stats::last_used.eq(now),
        ))
        .on_conflict((command_stats::command, command_stats::arguments))
        .do_update()
        .set((
            command_stats::count.eq(command_stats::count + 1),
            command_stats::last_used.eq(now),
        ))
        .execute(conn)?;

    Ok(())
}

/// Execute a command with timing and logging
pub async fn execute_command(
    ctx: &CommandContext,
    data: &Data,
    user: &User,
) -> Result<(), Box<dyn Error>> {
    // Log command execution
    log_command(&data.db_pool, &ctx.command, &ctx.args, user).await?;

    // Update command stats
    update_command_stats(&data.db_pool, &ctx.command, &ctx.args).await?;

    // Record metrics
    metrics::record_command_execution(&ctx.command);
    metrics::record_command_duration(&ctx.command, ctx.duration().as_secs_f64());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;

    mock! {
        PgConnection {}
        impl PgConnection {
            fn establish(url: &str) -> Result<Self, diesel::ConnectionError>;
        }
    }

    #[test]
    fn test_command_logging() {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool");
        let mut conn = pool.get().unwrap();
        let user = User::new(UserId::new(123));

        assert!(log_command(&mut conn, &user, "test_command").is_ok());
    }

    #[test]
    fn test_command_stats() {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool");
        let mut conn = pool.get().unwrap();
        let user = User::new(UserId::new(123));

        assert!(update_command_stats(&mut conn, "test_command", &user).is_ok());
    }

    #[test]
    fn test_command_context() {
        let ctx = CommandContext::new(
            "test".to_string(),
            vec!["arg1".to_string(), "arg2".to_string()],
        );
        assert_eq!(ctx.command, "test");
        assert_eq!(ctx.args, vec!["arg1", "arg2"]);
        assert!(ctx.duration() < Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_command_execution() {
        let pool = establish_connection();
        let mut user = User::default();
        user.id = UserId::new(123);

        let ctx = CommandContext::new("test".to_string(), vec!["arg1".to_string()]);
        let data = Data {
            db_pool: pool,
            ..Default::default()
        };

        assert!(execute_command(&ctx, &data, &user).await.is_ok());
    }
}
