mod commands;
mod models;
mod schema;
mod utils;
mod metrics;

// #[macro_use]
// extern crate diesel;
// use diesel::pg::Pg;
// use diesel::r2d2::ManageConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use poise::{
    self,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
// use std::error::Error;
use axum::{Router, response::Html};
use axum::routing::get;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use chrono::Utc;
use diesel::prelude::*;
use crate::models::CommandHistory;
use diesel_migrations::{MigrationHarness, EmbeddedMigrations, embed_migrations};
// All use statements above
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
use axum::serve;
use tokio::net::TcpListener;
use prometheus::{IntCounterVec, IntGauge, HistogramVec, register_int_counter_vec, register_histogram_vec, register_int_gauge, HistogramTimer};
use utils::{set_process_metrics, update_resource_metrics, update_discord_metrics, update_guild_metrics, prometheus_metrics};
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tracing::Level;
use tracing::error;
use std::collections::HashMap;
use tokio::sync::RwLock;
use poise::serenity_prelude::{GuildId, Guild, UserId, User, ChannelId, GuildChannel};

use commands::{
    advice::advice,
    ball::ball,
    botsnack::botsnack,
    desc::set,
    drink::drink,
    food::food,
    github::github,
    owner::quit,
    pingpong::{ping},
    random::random,
    stonks::{graph, stonkcomp, stonks},
    stats::stats,
};

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Poise user data and error type
type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub command_timers: Arc<RwLock<HashMap<String, f64>>>,
    pub guilds: Arc<RwLock<HashMap<GuildId, Guild>>>,
    pub users: Arc<RwLock<HashMap<UserId, User>>>,
    pub channels: Arc<RwLock<HashMap<ChannelId, GuildChannel>>>,
}

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
struct WebConfig {
    history_retention_days: i64,
}

// Global metrics registry
lazy_static::lazy_static! {
    static ref COMMAND_COUNTER: IntCounterVec = register_int_counter_vec!(
        "bot_command_total",
        "Total number of commands invoked",
        &["command"]
    ).unwrap();
    static ref COMMAND_FAILURES: IntCounterVec = register_int_counter_vec!(
        "bot_command_failures_total",
        "Total number of failed command invocations",
        &["command"]
    ).unwrap();
    static ref COMMAND_DURATION: HistogramVec = register_histogram_vec!(
        "bot_command_duration_seconds",
        "Command execution duration in seconds",
        &["command"]
    ).unwrap();
    static ref HTTP_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "bot_http_requests_total",
        "Total HTTP requests",
        &["endpoint", "method"]
    ).unwrap();
    static ref HTTP_DURATION: HistogramVec = register_histogram_vec!(
        "bot_http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["endpoint", "method"]
    ).unwrap();
    static ref PROCESS_START_TIME: IntGauge = register_int_gauge!(
        "process_start_time_seconds",
        "Process start time in seconds since epoch"
    ).unwrap();
    static ref DB_POOL_CONNECTIONS: IntGauge = register_int_gauge!(
        "bot_db_pool_connections",
        "Number of DB pool connections"
    ).unwrap();
    static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "bot_memory_usage_bytes",
        "Memory usage of the bot process in bytes"
    ).unwrap();
    static ref CPU_USAGE: IntGauge = register_int_gauge!(
        "bot_cpu_usage_percent",
        "CPU usage of the bot process in percent (0-100)"
    ).unwrap();
    static ref DISCORD_GUILD_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_guild_count",
        "Number of Discord guilds (servers) the bot is in"
    ).unwrap();
    static ref DISCORD_USER_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_user_count",
        "Number of Discord users visible to the bot"
    ).unwrap();
    static ref DISCORD_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_channel_count",
        "Number of Discord channels visible to the bot"
    ).unwrap();
    static ref GUILD_MEMBER_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_member_count",
        "Number of members in each guild"
    ).unwrap();
    static ref GUILD_CHANNEL_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_channel_count",
        "Number of channels in each guild"
    ).unwrap();
    static ref GUILD_ROLE_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_role_count",
        "Number of roles in each guild"
    ).unwrap();
    static ref GUILD_ONLINE_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_online_count",
        "Number of online members in each guild"
    ).unwrap();
    static ref GUILD_CREATION_TIME: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_creation_time_seconds",
        "Guild creation time in seconds since epoch"
    ).unwrap();
    static ref GUILD_HUMAN_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_human_count",
        "Number of human members in each guild"
    ).unwrap();
    static ref GUILD_BOT_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_bot_count",
        "Number of bot members in each guild"
    ).unwrap();
    static ref GUILD_TEXT_CHANNEL_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_text_channel_count",
        "Number of text channels in each guild"
    ).unwrap();
    static ref GUILD_VOICE_CHANNEL_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_voice_channel_count",
        "Number of voice channels in each guild"
    ).unwrap();
    static ref GUILD_CATEGORY_CHANNEL_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_category_channel_count",
        "Number of category channels in each guild"
    ).unwrap();
    static ref GUILD_EMOJI_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_emoji_count",
        "Number of emojis in each guild"
    ).unwrap();
    static ref GUILD_STICKER_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_sticker_count",
        "Number of stickers in each guild"
    ).unwrap();
    static ref GUILD_BOOST_COUNT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_boost_count",
        "Number of boosts in each guild"
    ).unwrap();
    static ref GUILD_PREMIUM_TIER: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_premium_tier",
        "Premium tier of each guild"
    ).unwrap();
    static ref GUILD_OWNER_ID: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_owner_id",
        "Owner ID of each guild (gauge set to 1 for the owner, 0 otherwise)"
    ).unwrap();
    static ref GUILD_AFK_TIMEOUT: prometheus::IntGauge = register_int_gauge!(
        "bot_guild_afk_timeout_seconds",
        "AFK timeout in seconds for each guild"
    ).unwrap();
}

async fn bot_info() -> Html<String> {
    Html(format!("<h1>TestBot</h1><p>Configuration: ...</p>"))
}

async fn command_history_handler(pool: axum::extract::Extension<Arc<TokioMutex<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>>>, config: axum::extract::Extension<WebConfig>) -> Html<String> {
    let pool = pool.lock().await;
    let mut conn = pool.get().unwrap();
    let cutoff = Utc::now().naive_utc() - chrono::Duration::days(config.history_retention_days);
    use crate::schema::command_history::dsl::*;
    let history: Vec<CommandHistory> = command_history
        .filter(timestamp.ge(cutoff))
        .order(timestamp.desc())
        .load(&mut conn)
        .unwrap_or_default();
    let html = history.iter().map(|h| format!("<li>[{}] {}: {}</li>", h.timestamp, h.user, h.command)).collect::<Vec<_>>().join("");
    Html(format!("<h2>Command History (last {} days)</h2><ul>{}</ul>", config.history_retention_days, html))
}

async fn stats_handler(_pool: axum::extract::Extension<Arc<TokioMutex<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>>>) -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Command Usage Stats</title>
    <style>
        body { font-family: 'Segoe UI', Arial, sans-serif; background: #f4f6fb; margin: 0; padding: 0; }
        .container { max-width: 900px; margin: 40px auto; background: #fff; border-radius: 12px; box-shadow: 0 2px 12px rgba(0,0,0,0.08); padding: 32px; }
        h2 { text-align: center; color: #2d3a4b; margin-bottom: 24px; }
        #refresh-btn { display: block; margin: 0 auto 24px auto; padding: 10px 24px; background: #4f8cff; color: #fff; border: none; border-radius: 6px; font-size: 1rem; cursor: pointer; transition: background 0.2s; }
        #refresh-btn:hover { background: #2563eb; }
        table { width: 100%; border-collapse: collapse; background: #fff; }
        th, td { padding: 12px 10px; text-align: left; }
        th { background: #f0f4fa; color: #2d3a4b; font-weight: 600; }
        tr:nth-child(even) { background: #f9fbfd; }
        tr:hover { background: #eaf1fb; }
        .args { color: #6b7280; font-size: 0.95em; }
        @media (max-width: 600px) {
            .container { padding: 10px; }
            th, td { padding: 8px 4px; font-size: 0.95em; }
        }
    </style>
</head>
<body>
    <div class=\"container\">
        <h2>Command Usage Stats</h2>
        <button id=\"refresh-btn\">Refresh</button>
        <div id=\"stats-table\">
            <!-- Table will be loaded here -->
        </div>
    </div>
    <script>
        async function loadStats() {
            const resp = await fetch('/stats/data');
            const html = await resp.text();
            document.getElementById('stats-table').innerHTML = html;
        }
        document.getElementById('refresh-btn').onclick = loadStats;
        window.onload = loadStats;
    </script>
</body>
</html>
"#;
    Html(html.to_string())
}

// New handler for /stats/data (returns just the table rows)
async fn stats_data_handler(pool: axum::extract::Extension<Arc<TokioMutex<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>>>) -> Html<String> {
    let pool = pool.lock().await;
    let mut conn = pool.get().unwrap();
    use crate::schema::command_stats::dsl::*;
    let stats: Vec<crate::models::CommandStat> = command_stats
        .order(count.desc())
        .limit(50)
        .load(&mut conn)
        .unwrap_or_default();
    let html = format!(
        "<table><tr><th>Command</th><th>Arguments</th><th>Count</th><th>Last Used</th></tr>{}</table>",
        stats.iter().map(|s| format!(
            "<tr><td>{}</td><td class='args'>{}</td><td>{}</td><td>{}</td></tr>",
            s.command, s.arguments, s.count, s.last_used
        )).collect::<Vec<_>>().join("\n")
    );
    Html(html)
}

// Add a /metrics endpoint for Prometheus
async fn metrics_handler() -> Html<String> {
    Html(prometheus_metrics())
}

// Add this before the main function
struct Handler;

#[poise::serenity_prelude::async_trait]
impl poise::serenity_prelude::EventHandler for Handler {
    async fn ready(&self, ctx: poise::serenity_prelude::Context, _ready: poise::serenity_prelude::Ready) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn guild_create(&self, ctx: poise::serenity_prelude::Context, _guild: poise::serenity_prelude::Guild, _is_new: Option<bool>) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn guild_delete(&self, ctx: poise::serenity_prelude::Context, _incomplete: poise::serenity_prelude::UnavailableGuild, _full: Option<poise::serenity_prelude::Guild>) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn guild_update(&self, ctx: poise::serenity_prelude::Context, _old: Option<poise::serenity_prelude::Guild>, _new: poise::serenity_prelude::PartialGuild) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn channel_create(&self, ctx: poise::serenity_prelude::Context, _channel: poise::serenity_prelude::GuildChannel) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn channel_delete(&self, ctx: poise::serenity_prelude::Context, _channel: poise::serenity_prelude::GuildChannel, _messages: Option<Vec<poise::serenity_prelude::Message>>) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn channel_update(&self, ctx: poise::serenity_prelude::Context, _old: Option<poise::serenity_prelude::GuildChannel>, _new: poise::serenity_prelude::GuildChannel) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }

    async fn user_update(&self, ctx: poise::serenity_prelude::Context, _old: Option<poise::serenity_prelude::CurrentUser>, _new: poise::serenity_prelude::CurrentUser) {
        update_discord_metrics(&ctx);
        update_guild_metrics(&ctx);
    }
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e);
            }
        }
    }
}

// --- Poise bot entry point ---
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN")?;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    set_process_metrics(&db_pool);
    // Spawn a background task to update resource metrics every second
    let db_pool_clone = db_pool.clone();
    tokio::spawn(async move {
        loop {
            update_resource_metrics(&db_pool_clone);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    // Run migrations automatically
    {
        let mut conn = db_pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run database migrations");
    }
    let history_retention_days = std::env::var("HISTORY_RETENTION_DAYS").ok().and_then(|s| s.parse().ok()).unwrap_or(30);
    let web_config = WebConfig { history_retention_days };
    let pool = Arc::new(TokioMutex::new(db_pool.clone()));
    // Prune old command history
    {
        let mut conn = db_pool.get().unwrap();
        let _ = crate::utils::prune_command_history(&mut conn, history_retention_days);
    }
    let options = poise::FrameworkOptions {
        commands: vec![
            advice(),
            ball(),
            botsnack(),
            set(),
            drink(),
            food(),
            github(),
            quit(),
            ping(),
            random(),
            stonks(),
            stonkcomp(),
            graph(),
            stats(),
        ],
        pre_command: |ctx| Box::pin(async move {
            let pool = &ctx.data().db_pool;
            let mut conn = pool.get().unwrap();
            let command = ctx.command().name.clone();
            let user = ctx.author().id.to_string();
            crate::commands::log_command(&mut conn, &user, &command);
            crate::commands::update_command_stats(&mut conn, &command, &command);
            COMMAND_COUNTER.with_label_values(&[&command]).inc();
            let timer = COMMAND_DURATION.with_label_values(&[&command]).start_timer();
            let mut timers = ctx.data().command_timers.lock().await;
            timers.insert(command.clone(), timer);
        }),
        post_command: |ctx| Box::pin(async move {
            let command = ctx.command().name.clone();
            let mut timers = ctx.data().command_timers.lock().await;
            if let Some(timer) = timers.remove(&command) {
                timer.observe_duration();
            }
        }),
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };
    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |_ctx, _ready, _framework| {
            let db_pool = db_pool.clone();
            Box::pin(async move { Ok(Data { db_pool, command_timers: Arc::new(TokioMutex::new(HashMap::new())), guilds: Arc::new(RwLock::new(HashMap::new())), users: Arc::new(RwLock::new(HashMap::new())), channels: Arc::new(RwLock::new(HashMap::new())) }) })
        })
        .build();
    let mut client = ClientBuilder::new(
        token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .event_handler(Handler)
    .await?;
    client.start().await?;
    let web_port: u16 = std::env::var("WEB_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let app = Router::new()
        .route("/", get(bot_info))
        .route("/history", get(command_history_handler))
        .route("/stats", get(stats_handler))
        .route("/stats/data", get(stats_data_handler))
        .route("/metrics", get(metrics_handler))
        .layer(axum::extract::Extension(pool))
        .layer(axum::extract::Extension(web_config))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    let method = request.method().as_str();
                    let path = request.uri().path();
                    metrics::HTTP_REQUESTS.with_label_values(&[&path.to_string(), &method.to_string()]).inc();
                })
        );
    tokio::spawn(async move {
        let listener = TcpListener::bind(("0.0.0.0", web_port)).await.unwrap();
        serve(listener, app.into_make_service()).await.unwrap();
    });
    Ok(())
}

/*
// --- Old serenity::framework::standard main logic ---
// #[tokio::main]
// #[instrument]
// async fn main() {
//     ...
// }
*/
