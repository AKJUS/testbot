use lazy_static::lazy_static;
use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge, HistogramVec,
    IntCounterVec, IntGauge,
};

/// Macro to register a gauge metric
macro_rules! register_gauge {
    ($name:ident, $help:expr) => {
        pub static ref $name: IntGauge = register_int_gauge!(
            concat!("bot_", stringify!($name)),
            $help
        ).unwrap();
    };
}

/// Macro to register a counter vector metric
macro_rules! register_counter_vec {
    ($name:ident, $help:expr, $labels:expr) => {
        pub static ref $name: IntCounterVec = register_int_counter_vec!(
            concat!("bot_", stringify!($name)),
            $help,
            $labels
        ).unwrap();
    };
}

/// Macro to register a histogram vector metric
macro_rules! register_histogram_vec {
    ($name:ident, $help:expr, $labels:expr) => {
        pub static ref $name: HistogramVec = register_histogram_vec!(
            concat!("bot_", stringify!($name)),
            $help,
            $labels
        ).unwrap();
    };
}

// Global metrics registry
lazy_static! {
    // Command metrics
    pub mod command {
        use super::*;
        register_counter_vec!(
            COUNTER,
            "Total number of commands invoked",
            &["command"]
        );
        register_counter_vec!(
            FAILURES,
            "Total number of failed command invocations",
            &["command"]
        );
        register_histogram_vec!(
            DURATION,
            "Command execution duration in seconds",
            &["command"]
        );
    }

    // HTTP metrics
    pub mod http {
        use super::*;
        register_counter_vec!(
            REQUESTS,
            "Total HTTP requests",
            &["endpoint", "method"]
        );
        register_histogram_vec!(
            DURATION,
            "HTTP request duration in seconds",
            &["endpoint", "method"]
        );
    }

    // Process metrics
    pub mod process {
        use super::*;
        register_gauge!(
            START_TIME,
            "Process start time in seconds since epoch"
        );
        register_gauge!(
            DB_POOL_CONNECTIONS,
            "Number of DB pool connections"
        );
        register_gauge!(
            MEMORY_USAGE,
            "Memory usage of the bot process in bytes"
        );
        register_gauge!(
            CPU_USAGE,
            "CPU usage of the bot process in percent (0-100)"
        );
    }

    // Discord metrics
    pub mod discord {
        use super::*;
        register_gauge!(
            GUILD_COUNT,
            "Number of Discord guilds (servers) the bot is in"
        );
        register_gauge!(
            USER_COUNT,
            "Number of Discord users visible to the bot"
        );
        register_gauge!(
            CHANNEL_COUNT,
            "Number of Discord channels visible to the bot"
        );
    }

    // Guild metrics
    pub mod guild {
        use super::*;
        register_gauge!(
            MEMBER_COUNT,
            "Number of members in each guild"
        );
        register_gauge!(
            CHANNEL_COUNT,
            "Number of channels in each guild"
        );
        register_gauge!(
            ROLE_COUNT,
            "Number of roles in each guild"
        );
        register_gauge!(
            ONLINE_COUNT,
            "Number of online members in each guild"
        );
        register_gauge!(
            CREATION_TIME,
            "Guild creation time in seconds since epoch"
        );
        register_gauge!(
            HUMAN_COUNT,
            "Number of human members in each guild"
        );
        register_gauge!(
            BOT_COUNT,
            "Number of bot members in each guild"
        );
        register_gauge!(
            TEXT_CHANNEL_COUNT,
            "Number of text channels in each guild"
        );
        register_gauge!(
            VOICE_CHANNEL_COUNT,
            "Number of voice channels in each guild"
        );
        register_gauge!(
            CATEGORY_CHANNEL_COUNT,
            "Number of category channels in each guild"
        );
        register_gauge!(
            EMOJI_COUNT,
            "Number of emojis in each guild"
        );
        register_gauge!(
            STICKER_COUNT,
            "Number of stickers in each guild"
        );
        register_gauge!(
            BOOST_COUNT,
            "Number of boosts in each guild"
        );
        register_gauge!(
            PREMIUM_TIER,
            "Premium tier of each guild"
        );
        register_gauge!(
            OWNER_ID,
            "Owner ID of each guild (gauge set to 1 for the owner, 0 otherwise)"
        );
        register_gauge!(
            AFK_TIMEOUT,
            "AFK timeout in seconds for each guild"
        );
    }

    // Interaction metrics
    pub mod interaction {
        use super::*;
        // Slash command metrics
        register_counter_vec!(
            SLASH_COMMAND_USAGE,
            "Total number of slash commands used",
            &["command", "guild_id"]
        );
        register_histogram_vec!(
            SLASH_COMMAND_DURATION,
            "Time taken to process slash commands",
            &["command", "guild_id"]
        );
        register_counter_vec!(
            SLASH_COMMAND_FAILURES,
            "Number of failed slash command executions",
            &["command", "guild_id", "error_type"]
        );

        // Button interaction metrics
        register_counter_vec!(
            BUTTON_CLICKS,
            "Total number of button clicks",
            &["button_id", "guild_id"]
        );
        register_histogram_vec!(
            BUTTON_RESPONSE_TIME,
            "Time taken to respond to button clicks",
            &["button_id", "guild_id"]
        );
        register_counter_vec!(
            BUTTON_FAILURES,
            "Number of failed button interactions",
            &["button_id", "guild_id", "error_type"]
        );

        // Select menu metrics
        register_counter_vec!(
            SELECT_MENU_USAGE,
            "Total number of select menu interactions",
            &["menu_id", "guild_id"]
        );
        register_histogram_vec!(
            SELECT_MENU_RESPONSE_TIME,
            "Time taken to respond to select menu interactions",
            &["menu_id", "guild_id"]
        );
        register_counter_vec!(
            SELECT_MENU_FAILURES,
            "Number of failed select menu interactions",
            &["menu_id", "guild_id", "error_type"]
        );

        // Modal submission metrics
        register_counter_vec!(
            MODAL_SUBMISSIONS,
            "Total number of modal form submissions",
            &["modal_id", "guild_id"]
        );
        register_histogram_vec!(
            MODAL_PROCESSING_TIME,
            "Time taken to process modal submissions",
            &["modal_id", "guild_id"]
        );
        register_counter_vec!(
            MODAL_FAILURES,
            "Number of failed modal submissions",
            &["modal_id", "guild_id", "error_type"]
        );

        // Context menu metrics
        register_counter_vec!(
            CONTEXT_MENU_USAGE,
            "Total number of context menu interactions",
            &["command", "guild_id"]
        );
        register_histogram_vec!(
            CONTEXT_MENU_DURATION,
            "Time taken to process context menu commands",
            &["command", "guild_id"]
        );
        register_counter_vec!(
            CONTEXT_MENU_FAILURES,
            "Number of failed context menu interactions",
            &["command", "guild_id", "error_type"]
        );

        // Autocomplete metrics
        register_counter_vec!(
            AUTOCOMPLETE_REQUESTS,
            "Total number of autocomplete requests",
            &["command", "guild_id"]
        );
        register_histogram_vec!(
            AUTOCOMPLETE_RESPONSE_TIME,
            "Time taken to respond to autocomplete requests",
            &["command", "guild_id"]
        );
        register_counter_vec!(
            AUTOCOMPLETE_FAILURES,
            "Number of failed autocomplete requests",
            &["command", "guild_id", "error_type"]
        );

        // Interaction rate limiting metrics
        register_counter_vec!(
            RATE_LIMIT_HITS,
            "Number of times rate limits were hit",
            &["interaction_type", "guild_id"]
        );
        register_gauge!(
            ACTIVE_INTERACTIONS,
            "Number of currently active interactions"
        );
    }
}

// Re-export commonly used metrics
pub use command::{
    COUNTER as COMMAND_COUNTER, DURATION as COMMAND_DURATION, FAILURES as COMMAND_FAILURES,
};
pub use discord::{CHANNEL_COUNT, GUILD_COUNT, USER_COUNT};
pub use guild::{
    AFK_TIMEOUT, BOOST_COUNT, BOT_COUNT, CATEGORY_CHANNEL_COUNT,
    CHANNEL_COUNT as GUILD_CHANNEL_COUNT, CREATION_TIME, EMOJI_COUNT, HUMAN_COUNT, MEMBER_COUNT,
    ONLINE_COUNT, OWNER_ID, PREMIUM_TIER, ROLE_COUNT, STICKER_COUNT, TEXT_CHANNEL_COUNT,
    VOICE_CHANNEL_COUNT,
};
pub use http::{DURATION as HTTP_DURATION, REQUESTS as HTTP_REQUESTS};
pub use interaction::{
    ACTIVE_INTERACTIONS, AUTOCOMPLETE_FAILURES, AUTOCOMPLETE_REQUESTS, AUTOCOMPLETE_RESPONSE_TIME,
    BUTTON_CLICKS, BUTTON_FAILURES, BUTTON_RESPONSE_TIME, CONTEXT_MENU_DURATION,
    CONTEXT_MENU_FAILURES, CONTEXT_MENU_USAGE, MODAL_FAILURES, MODAL_PROCESSING_TIME,
    MODAL_SUBMISSIONS, RATE_LIMIT_HITS, SELECT_MENU_FAILURES, SELECT_MENU_RESPONSE_TIME,
    SELECT_MENU_USAGE, SLASH_COMMAND_DURATION, SLASH_COMMAND_FAILURES, SLASH_COMMAND_USAGE,
};
pub use process::{CPU_USAGE, DB_POOL_CONNECTIONS, MEMORY_USAGE, START_TIME as PROCESS_START_TIME};

pub fn record_http_request(endpoint: &str, method: &str) {
    HTTP_REQUESTS.with_label_values(&[endpoint, method]).inc();
}

pub fn record_command_execution(command: &str) {
    COMMAND_COUNTER.with_label_values(&[command]).inc();
}

pub fn record_command_duration(command: &str, duration: f64) {
    COMMAND_DURATION
        .with_label_values(&[command])
        .observe(duration);
}

pub fn update_guild_count(count: i64) {
    GUILD_COUNT.set(count);
}

pub fn update_member_count(count: i64) {
    MEMBER_COUNT.set(count);
}

// Helper functions for interaction metrics
pub fn record_slash_command(command: &str, guild_id: &str, duration: f64) {
    SLASH_COMMAND_USAGE
        .with_label_values(&[command, guild_id])
        .inc();
    SLASH_COMMAND_DURATION
        .with_label_values(&[command, guild_id])
        .observe(duration);
}

pub fn record_slash_command_failure(command: &str, guild_id: &str, error_type: &str) {
    SLASH_COMMAND_FAILURES
        .with_label_values(&[command, guild_id, error_type])
        .inc();
}

pub fn record_button_click(button_id: &str, guild_id: &str, response_time: f64) {
    BUTTON_CLICKS
        .with_label_values(&[button_id, guild_id])
        .inc();
    BUTTON_RESPONSE_TIME
        .with_label_values(&[button_id, guild_id])
        .observe(response_time);
}

pub fn record_button_failure(button_id: &str, guild_id: &str, error_type: &str) {
    BUTTON_FAILURES
        .with_label_values(&[button_id, guild_id, error_type])
        .inc();
}

pub fn record_select_menu_usage(menu_id: &str, guild_id: &str, response_time: f64) {
    SELECT_MENU_USAGE
        .with_label_values(&[menu_id, guild_id])
        .inc();
    SELECT_MENU_RESPONSE_TIME
        .with_label_values(&[menu_id, guild_id])
        .observe(response_time);
}

pub fn record_select_menu_failure(menu_id: &str, guild_id: &str, error_type: &str) {
    SELECT_MENU_FAILURES
        .with_label_values(&[menu_id, guild_id, error_type])
        .inc();
}

pub fn record_modal_submission(modal_id: &str, guild_id: &str, processing_time: f64) {
    MODAL_SUBMISSIONS
        .with_label_values(&[modal_id, guild_id])
        .inc();
    MODAL_PROCESSING_TIME
        .with_label_values(&[modal_id, guild_id])
        .observe(processing_time);
}

pub fn record_modal_failure(modal_id: &str, guild_id: &str, error_type: &str) {
    MODAL_FAILURES
        .with_label_values(&[modal_id, guild_id, error_type])
        .inc();
}

pub fn record_context_menu_usage(command: &str, guild_id: &str, duration: f64) {
    CONTEXT_MENU_USAGE
        .with_label_values(&[command, guild_id])
        .inc();
    CONTEXT_MENU_DURATION
        .with_label_values(&[command, guild_id])
        .observe(duration);
}

pub fn record_context_menu_failure(command: &str, guild_id: &str, error_type: &str) {
    CONTEXT_MENU_FAILURES
        .with_label_values(&[command, guild_id, error_type])
        .inc();
}

pub fn record_autocomplete_request(command: &str, guild_id: &str, response_time: f64) {
    AUTOCOMPLETE_REQUESTS
        .with_label_values(&[command, guild_id])
        .inc();
    AUTOCOMPLETE_RESPONSE_TIME
        .with_label_values(&[command, guild_id])
        .observe(response_time);
}

pub fn record_autocomplete_failure(command: &str, guild_id: &str, error_type: &str) {
    AUTOCOMPLETE_FAILURES
        .with_label_values(&[command, guild_id, error_type])
        .inc();
}

pub fn record_rate_limit_hit(interaction_type: &str, guild_id: &str) {
    RATE_LIMIT_HITS
        .with_label_values(&[interaction_type, guild_id])
        .inc();
}

pub fn update_active_interactions(count: i64) {
    ACTIVE_INTERACTIONS.set(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        // Test HTTP request recording
        record_http_request("/test", "GET");
        assert_eq!(HTTP_REQUESTS.with_label_values(&["/test", "GET"]).get(), 1);

        // Test command execution recording
        record_command_execution("test_command");
        assert_eq!(
            COMMAND_COUNTER.with_label_values(&["test_command"]).get(),
            1
        );

        // Test command duration recording
        record_command_duration("test_command", 1.5);
        let duration = COMMAND_DURATION
            .with_label_values(&["test_command"])
            .get_sample_sum();
        assert!(duration > 0.0);

        // Test guild and member count updates
        update_guild_count(5);
        assert_eq!(GUILD_COUNT.get(), 5);

        update_member_count(100);
        assert_eq!(MEMBER_COUNT.get(), 100);

        // Test interaction metrics
        record_slash_command("test_command", "123", 0.5);
        assert_eq!(
            SLASH_COMMAND_USAGE
                .with_label_values(&["test_command", "123"])
                .get(),
            1
        );

        record_button_click("test_button", "123", 0.2);
        assert_eq!(
            BUTTON_CLICKS
                .with_label_values(&["test_button", "123"])
                .get(),
            1
        );

        record_select_menu_usage("test_menu", "123", 0.3);
        assert_eq!(
            SELECT_MENU_USAGE
                .with_label_values(&["test_menu", "123"])
                .get(),
            1
        );

        record_modal_submission("test_modal", "123", 0.4);
        assert_eq!(
            MODAL_SUBMISSIONS
                .with_label_values(&["test_modal", "123"])
                .get(),
            1
        );

        record_context_menu_usage("test_context", "123", 0.6);
        assert_eq!(
            CONTEXT_MENU_USAGE
                .with_label_values(&["test_context", "123"])
                .get(),
            1
        );

        record_autocomplete_request("test_command", "123", 0.1);
        assert_eq!(
            AUTOCOMPLETE_REQUESTS
                .with_label_values(&["test_command", "123"])
                .get(),
            1
        );

        record_rate_limit_hit("slash_command", "123");
        assert_eq!(
            RATE_LIMIT_HITS
                .with_label_values(&["slash_command", "123"])
                .get(),
            1
        );

        update_active_interactions(5);
        assert_eq!(ACTIVE_INTERACTIONS.get(), 5);
    }
}
