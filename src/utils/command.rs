use poise::Context;
use crate::Error;
use crate::metrics::command;

/// Log command usage to metrics
pub async fn log_command_usage(ctx: &Context<'_, crate::Data, Error>, command_name: &str, args: &str) -> Result<(), Error> {
    // Increment command counter
    command::COUNTER.with_label_values(&[command_name]).inc();

    // Log command usage to database
    if let Some(guild_id) = ctx.guild_id() {
        let mut conn = ctx.data().pool.get().await?;
        let user = ctx.author().name.clone();
        let mut cmd_ctx = crate::commands::CommandContext::new(&mut conn, &user, command_name, args);
        cmd_ctx.log_command();
        cmd_ctx.update_stats();
    }

    Ok(())
}

/// Log command failure to metrics
pub fn log_command_failure(command_name: &str) {
    command::FAILURES.with_label_values(&[command_name]).inc();
}

/// Start a command duration timer
pub fn start_command_timer(command_name: &str) -> prometheus::HistogramTimer {
    command::DURATION.with_label_values(&[command_name]).start_timer()
}

/// Log HTTP request to metrics
pub fn log_http_request(endpoint: &str, method: &str) {
    crate::metrics::http::REQUESTS.with_label_values(&[endpoint, method]).inc();
}

/// Start an HTTP request duration timer
pub fn start_http_timer(endpoint: &str, method: &str) -> prometheus::HistogramTimer {
    crate::metrics::http::DURATION.with_label_values(&[endpoint, method]).start_timer()
} 