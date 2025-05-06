use crate::db::DbPool;
use crate::db::Pool;
use crate::metrics::{
    COMMAND_REQUESTS,
    COMMAND_ERRORS,
    COMMAND_DURATION,
};
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
use poise::serenity_prelude::{Context, User, UserId};
use std::error::Error;
use std::time::{Duration, Instant};
use crate::utils::command::CommandContext;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::interactions::InteractionTracker;

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
    pub command_name: String,
    pub args: Vec<String>,
    start_time: Instant,
}

impl CommandContext {
    pub fn new(command_name: String, args: Vec<String>) -> Self {
        Self {
            command_name,
            args,
            start_time: Instant::now(),
        }
    }

    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn log_command(&self, conn: &mut PgConnection, user: &User) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(command_history::table)
            .values((
                command_history::command.eq(&self.command_name),
                command_history::user_id.eq(user.id.get() as i64),
                command_history::guild_id.eq(None::<i64>),
                command_history::timestamp.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_stats(&self, conn: &mut PgConnection, user: &User) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(command_stats::table)
            .values((
                command_stats::command.eq(&self.command_name),
                command_stats::count.eq(1),
            ))
            .on_conflict(command_stats::command)
            .do_update()
            .set(command_stats::count.eq(command_stats::count + 1))
            .execute(conn)?;
        Ok(())
    }
}

/// Log a command execution to the database
pub async fn log_command(
    pool: &Pool<ConnectionManager<PgConnection>>,
    command: &str,
    args: &[String],
    user: &User,
) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get()?;

    diesel::insert_into(command_history::table)
        .values((
            command_history::command.eq(command),
            command_history::arguments.eq(args.join(" ")),
            command_history::user_id.eq(user.id.get() as i64),
            command_history::executed_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut *conn)?;

    Ok(())
}

/// Update command statistics in the database
pub async fn update_command_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
    command: &str,
    args: &[String],
) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get()?;
    let now = Utc::now().naive_utc();

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
        .execute(&mut *conn)?;

    Ok(())
}

/// Execute a command with timing and logging
pub async fn execute_command(
    ctx: &CommandContext,
    data: &Data,
    user: &User,
) -> Result<(), Box<dyn Error>> {
    let mut conn = data.db_pool.get()?;
    
    // Record metrics
    COMMAND_REQUESTS.with_label_values(&[&ctx.command_name]).inc();
    
    // Log command execution
    ctx.log_command(&mut conn, user)?;

    // Update command stats
    ctx.update_stats(&mut conn, user)?;

    // Record duration
    let duration = ctx.duration().as_secs_f64();
    COMMAND_DURATION.with_label_values(&[&ctx.command_name]).observe(duration);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use std::collections::HashMap;

    fn create_test_pool() -> Pool<ConnectionManager<PgConnection>> {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool")
    }

    #[tokio::test]
    async fn test_command_logging() {
        let pool = create_test_pool();
        let user = User {
            id: UserId::new(123),
            name: "test_user".to_string(),
            discriminator: None,
            avatar: None,
            bot: false,
            system: false,
            mfa_enabled: false,
            verified: false,
            email: None,
            flags: None,
            premium_type: None,
            public_flags: None,
            accent_color: None,
            global_name: None,
            avatar_decoration: None,
            display_name: None,
            banner: None,
        };

        let result = log_command(&pool, "test_command", &["arg1".to_string()], &user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_command_stats() {
        let pool = create_test_pool();
        let result = update_command_stats(&pool, "test_command", &["arg1".to_string()]).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_context() {
        let ctx = CommandContext::new("test_command".to_string(), vec!["arg1".to_string()]);
        assert_eq!(ctx.command_name, "test_command");
        assert_eq!(ctx.args, vec!["arg1"]);
    }

    #[test]
    fn test_command_duration() {
        let ctx = CommandContext::new("test_command".to_string(), vec![]);
        std::thread::sleep(Duration::from_millis(100));
        assert!(ctx.duration() >= Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_command_execution() {
        let pool = create_test_pool();
        let user = User {
            id: UserId::new(123),
            name: "test_user".to_string(),
            discriminator: None,
            avatar: None,
            bot: false,
            system: false,
            mfa_enabled: false,
            verified: false,
            email: None,
            flags: None,
            premium_type: None,
            public_flags: None,
            accent_color: None,
            global_name: None,
            avatar_decoration: None,
            display_name: None,
            banner: None,
        };

        let ctx = CommandContext::new("test".to_string(), vec!["arg1".to_string()]);
        let data = Data {
            db_pool: Arc::new(pool),
            command_timers: HashMap::new(),
            guilds: Arc::new(HashMap::new()),
            users: Arc::new(HashMap::new()),
            channels: Arc::new(HashMap::new()),
            interaction_tracker: RwLock::new(InteractionTracker::new(Arc::new(create_test_pool()))),
        };

        assert!(execute_command(&ctx, &data, &user).await.is_ok());
    }
}
