use crate::metrics::{
    COMMAND_DURATION, COMMAND_ERRORS, COMMAND_REQUESTS,
    HTTP_DURATION, HTTP_REQUESTS,
};
use crate::Error;
use poise::Context;
use crate::Data;
use poise::serenity_prelude::User;
use std::error::Error;
use diesel::prelude::*;
use crate::schema::{command_logs, command_stats};
use chrono::Utc;

/// Log command usage to metrics
pub async fn log_command_usage(
    ctx: &Context<'_, crate::Data, Error>,
    command_name: &str,
    args: &str,
) -> Result<(), Error> {
    // Increment command counter
    command::COUNTER.with_label_values(&[command_name]).inc();

    // Log command usage to database
    if let Some(guild_id) = ctx.guild_id() {
        let mut conn = ctx.data().db_pool.get().await?;
        let user = ctx.author().name.clone();
        let mut cmd_ctx = crate::commands::CommandContext::new(
            command_name.to_string(),
            args.split_whitespace().map(|s| s.to_string()).collect(),
        );
        cmd_ctx.log_command(&mut conn, &user)?;
        cmd_ctx.update_stats(&mut conn, &user)?;
    }

    Ok(())
}

/// Log command failure to metrics
pub fn log_command_failure(command_name: &str) {
    command::FAILURES.with_label_values(&[command_name]).inc();
}

/// Start a command duration timer
pub fn start_command_timer(command_name: &str) -> prometheus::HistogramTimer {
    command::DURATION
        .with_label_values(&[command_name])
        .start_timer()
}

/// Log HTTP request to metrics
pub fn log_http_request(endpoint: &str, method: &str) {
    crate::metrics::http::REQUESTS
        .with_label_values(&[endpoint, method])
        .inc();
}

/// Start an HTTP request duration timer
pub fn start_http_timer(endpoint: &str, method: &str) -> prometheus::HistogramTimer {
    crate::metrics::http::DURATION
        .with_label_values(&[endpoint, method])
        .start_timer()
}

pub async fn handle_command(
    ctx: &poise::Context<'_, Data, Box<dyn std::error::Error + Send + Sync>>,
    user: &User,
    command_name: &str,
    args: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = ctx.data().db_pool.get()?;
    
    // Log command execution
    diesel::insert_into(command_logs::table)
        .values((
            command_logs::command.eq(command_name),
            command_logs::user_id.eq(user.id.get() as i64),
            command_logs::timestamp.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut *conn)?;

    // Update command stats
    diesel::insert_into(command_stats::table)
        .values((
            command_stats::command.eq(command_name),
            command_stats::count.eq(1),
        ))
        .on_conflict(command_stats::command)
        .do_update()
        .set(command_stats::count.eq(command_stats::count + 1))
        .execute(&mut *conn)?;

    Ok(())
}

pub struct CommandContext {
    pub command_name: String,
    pub args: Vec<String>,
}

impl CommandContext {
    pub fn new(command_name: String, args: Vec<String>) -> Self {
        Self {
            command_name,
            args,
        }
    }

    pub fn log_command(&self, conn: &mut diesel::PgConnection, user: &User) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(command_logs::table)
            .values((
                command_logs::command.eq(&self.command_name),
                command_logs::user_id.eq(user.id.to_string()),
                command_logs::timestamp.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_stats(&self, conn: &mut diesel::PgConnection, user: &User) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(command_logs::table)
            .values((
                command_logs::command.eq(&self.command_name),
                command_logs::user_id.eq(user.id.to_string()),
                command_logs::timestamp.eq(now),
            ))
            .on_conflict((command_logs::command, command_logs::user_id))
            .do_update()
            .set(command_logs::timestamp.eq(now))
            .execute(conn)?;
        Ok(())
    }
}
