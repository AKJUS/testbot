use prometheus::{Encoder, TextEncoder, IntCounterVec, IntGauge, HistogramVec, register_int_counter_vec, register_histogram_vec, register_int_gauge, gather};
use lazy_static::lazy_static;

// Global metrics registry
lazy_static! {
    pub static ref COMMAND_COUNTER: IntCounterVec = register_int_counter_vec!(
        "bot_command_total",
        "Total number of commands invoked",
        &["command"]
    ).unwrap();
    pub static ref COMMAND_FAILURES: IntCounterVec = register_int_counter_vec!(
        "bot_command_failures_total",
        "Total number of failed command invocations",
        &["command"]
    ).unwrap();
    pub static ref COMMAND_DURATION: HistogramVec = register_histogram_vec!(
        "bot_command_duration_seconds",
        "Command execution duration in seconds",
        &["command"]
    ).unwrap();
    pub static ref HTTP_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "bot_http_requests_total",
        "Total HTTP requests",
        &["endpoint", "method"]
    ).unwrap();
    pub static ref HTTP_DURATION: HistogramVec = register_histogram_vec!(
        "bot_http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["endpoint", "method"]
    ).unwrap();
    pub static ref PROCESS_START_TIME: IntGauge = register_int_gauge!(
        "process_start_time_seconds",
        "Process start time in seconds since epoch"
    ).unwrap();
    pub static ref DB_POOL_CONNECTIONS: IntGauge = register_int_gauge!(
        "bot_db_pool_connections",
        "Number of DB pool connections"
    ).unwrap();
    pub static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "bot_memory_usage_bytes",
        "Memory usage of the bot process in bytes"
    ).unwrap();
    pub static ref CPU_USAGE: IntGauge = register_int_gauge!(
        "bot_cpu_usage_percent",
        "CPU usage of the bot process in percent (0-100)"
    ).unwrap();
    pub static ref DISCORD_GUILD_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_guild_count",
        "Number of Discord guilds (servers) the bot is in"
    ).unwrap();
    pub static ref DISCORD_USER_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_user_count",
        "Number of Discord users visible to the bot"
    ).unwrap();
    pub static ref DISCORD_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_discord_channel_count",
        "Number of Discord channels visible to the bot"
    ).unwrap();
    pub static ref GUILD_MEMBER_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_member_count",
        "Number of members in each guild"
    ).unwrap();
    pub static ref GUILD_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_channel_count",
        "Number of channels in each guild"
    ).unwrap();
    pub static ref GUILD_ROLE_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_role_count",
        "Number of roles in each guild"
    ).unwrap();
    pub static ref GUILD_ONLINE_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_online_count",
        "Number of online members in each guild"
    ).unwrap();
    pub static ref GUILD_CREATION_TIME: IntGauge = register_int_gauge!(
        "bot_guild_creation_time_seconds",
        "Guild creation time in seconds since epoch"
    ).unwrap();
    pub static ref GUILD_HUMAN_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_human_count",
        "Number of human members in each guild"
    ).unwrap();
    pub static ref GUILD_BOT_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_bot_count",
        "Number of bot members in each guild"
    ).unwrap();
    pub static ref GUILD_TEXT_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_text_channel_count",
        "Number of text channels in each guild"
    ).unwrap();
    pub static ref GUILD_VOICE_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_voice_channel_count",
        "Number of voice channels in each guild"
    ).unwrap();
    pub static ref GUILD_CATEGORY_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_category_channel_count",
        "Number of category channels in each guild"
    ).unwrap();
    pub static ref GUILD_EMOJI_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_emoji_count",
        "Number of emojis in each guild"
    ).unwrap();
    pub static ref GUILD_STICKER_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_sticker_count",
        "Number of stickers in each guild"
    ).unwrap();
    pub static ref GUILD_BOOST_COUNT: IntGauge = register_int_gauge!(
        "bot_guild_boost_count",
        "Number of boosts in each guild"
    ).unwrap();
    pub static ref GUILD_PREMIUM_TIER: IntGauge = register_int_gauge!(
        "bot_guild_premium_tier",
        "Premium tier of each guild"
    ).unwrap();
    pub static ref GUILD_OWNER_ID: IntGauge = register_int_gauge!(
        "bot_guild_owner_id",
        "Owner ID of each guild (gauge set to 1 for the owner, 0 otherwise)"
    ).unwrap();
    pub static ref GUILD_AFK_TIMEOUT: IntGauge = register_int_gauge!(
        "bot_guild_afk_timeout_seconds",
        "AFK timeout in seconds for each guild"
    ).unwrap();
}

pub fn prometheus_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}