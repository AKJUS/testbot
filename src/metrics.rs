use lazy_static::lazy_static;
use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec, IntCounter,
    IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry,
};
use std::error::Error;

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

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // Command metrics
    pub static ref COMMAND_REQUESTS: IntCounterVec = IntCounterVec::new(
        Opts::new("command_requests_total", "Total number of command requests"),
        &["command"]
    )
    .unwrap();

    pub static ref COMMAND_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("command_duration_seconds", "Command execution duration in seconds"),
        &["command"]
    )
    .unwrap();

    pub static ref COMMAND_ERRORS: IntCounterVec = IntCounterVec::new(
        Opts::new("command_errors_total", "Total number of command errors"),
        &["command"]
    )
    .unwrap();

    // HTTP metrics
    pub static ref HTTP_REQUESTS: IntCounterVec = IntCounterVec::new(
        Opts::new("http_requests_total", "Total number of HTTP requests"),
        &["path", "method"]
    )
    .unwrap();

    pub static ref HTTP_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("http_duration_seconds", "HTTP request duration in seconds"),
        &["path", "method"]
    )
    .unwrap();

    // System metrics
    pub static ref MEMORY_USAGE: Gauge = Gauge::new(
        "memory_usage_bytes",
        "Current memory usage in bytes"
    )
    .unwrap();

    pub static ref CPU_USAGE: Gauge = Gauge::new(
        "cpu_usage_percent",
        "Current CPU usage percentage"
    )
    .unwrap();

    pub static ref GUILD_COUNT: IntGauge = IntGauge::new(
        "guild_count",
        "Total number of guilds"
    )
    .unwrap();

    pub static ref MEMBER_COUNT: IntGauge = IntGauge::new(
        "member_count",
        "Total number of members"
    )
    .unwrap();

    // Process metrics
    pub static ref DB_POOL_CONNECTIONS: IntGauge = register_int_gauge!(
        "db_pool_connections",
        "Number of database pool connections"
    )
    .unwrap();

    pub static ref PROCESS_START_TIME: IntGauge = register_int_gauge!(
        "process_start_time_seconds",
        "Process start time in seconds since epoch"
    )
    .unwrap();

    // Discord metrics
    pub static ref USER_COUNT: IntGauge = register_int_gauge!(
        "user_count",
        "Total number of users"
    )
    .unwrap();

    pub static ref CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "channel_count",
        "Total number of channels"
    )
    .unwrap();

    // Guild metrics
    pub static ref TEXT_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "text_channel_count",
        "Total number of text channels"
    )
    .unwrap();

    pub static ref VOICE_CHANNEL_COUNT: IntGauge = register_int_gauge!(
        "voice_channel_count",
        "Total number of voice channels"
    )
    .unwrap();

    pub static ref CATEGORY_COUNT: IntGauge = register_int_gauge!(
        "category_count",
        "Total number of categories"
    )
    .unwrap();

    pub static ref ROLE_COUNT: IntGauge = register_int_gauge!(
        "role_count",
        "Total number of roles"
    )
    .unwrap();

    pub static ref EMOJI_COUNT: IntGauge = register_int_gauge!(
        "emoji_count",
        "Total number of emojis"
    )
    .unwrap();

    pub static ref BOOST_COUNT: IntGauge = register_int_gauge!(
        "boost_count",
        "Total number of boosts"
    )
    .unwrap();

    pub static ref BOOST_LEVEL: IntGauge = register_int_gauge!(
        "boost_level",
        "Current boost level"
    )
    .unwrap();

    // Interaction metrics
    pub static ref INTERACTION_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "interaction_requests_total",
        "Total number of interaction requests",
        &["type"]
    )
    .unwrap();

    pub static ref INTERACTION_ERRORS: IntCounterVec = register_int_counter_vec!(
        "interaction_errors_total",
        "Total number of interaction errors",
        &["type"]
    )
    .unwrap();

    pub static ref INTERACTION_DURATION: HistogramVec = register_histogram_vec!(
        "interaction_duration_seconds",
        "Interaction execution duration in seconds",
        &["type"]
    )
    .unwrap();
}

pub fn register_metrics() -> Result<(), Box<dyn Error>> {
    REGISTRY.register(Box::new(COMMAND_REQUESTS.clone()))?;
    REGISTRY.register(Box::new(COMMAND_DURATION.clone()))?;
    REGISTRY.register(Box::new(COMMAND_ERRORS.clone()))?;
    REGISTRY.register(Box::new(HTTP_REQUESTS.clone()))?;
    REGISTRY.register(Box::new(HTTP_DURATION.clone()))?;
    REGISTRY.register(Box::new(MEMORY_USAGE.clone()))?;
    REGISTRY.register(Box::new(CPU_USAGE.clone()))?;
    REGISTRY.register(Box::new(GUILD_COUNT.clone()))?;
    REGISTRY.register(Box::new(MEMBER_COUNT.clone()))?;
    Ok(())
}

pub fn record_http_request(method: &str, path: &str) {
    HTTP_REQUESTS.with_label_values(&[path, method]).inc();
}

pub fn record_http_duration(method: &str, path: &str, duration: f64) {
    HTTP_DURATION.with_label_values(&[path, method]).observe(duration);
}

pub fn record_command_execution(command: &str) {
    COMMAND_REQUESTS.with_label_values(&[command]).inc();
}

pub fn record_command_error(command: &str) {
    COMMAND_ERRORS.with_label_values(&[command]).inc();
}

pub fn record_command_duration(command: &str, duration: f64) {
    COMMAND_DURATION.with_label_values(&[command]).observe(duration);
}

pub fn update_guild_count(count: i64) {
    GUILD_COUNT.set(count);
}

pub fn update_member_count(count: i64) {
    MEMBER_COUNT.set(count);
}

pub fn record_interaction(interaction_type: &str) {
    INTERACTION_REQUESTS.with_label_values(&[interaction_type]).inc();
}

pub fn record_interaction_error(interaction_type: &str) {
    INTERACTION_ERRORS.with_label_values(&[interaction_type]).inc();
}

pub fn record_interaction_duration(interaction_type: &str, duration: f64) {
    INTERACTION_DURATION.with_label_values(&[interaction_type]).observe(duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_metrics_registration() {
        assert!(register_metrics().is_ok());
    }

    #[test]
    fn test_command_metrics() {
        COMMAND_REQUESTS.with_label_values(&["test"]).inc();
        assert_eq!(COMMAND_REQUESTS.with_label_values(&["test"]).get(), 1);

        COMMAND_DURATION.with_label_values(&["test"]).observe(0.5);
        let duration = COMMAND_DURATION.with_label_values(&["test"]).get_sample_sum();
        assert!(duration > 0.4 && duration < 0.6);

        COMMAND_ERRORS.with_label_values(&["test"]).inc();
        assert_eq!(COMMAND_ERRORS.with_label_values(&["test"]).get(), 1);
    }

    #[test]
    fn test_http_metrics() {
        HTTP_REQUESTS.with_label_values(&["/test", "GET"]).inc();
        assert_eq!(HTTP_REQUESTS.with_label_values(&["/test", "GET"]).get(), 1);

        HTTP_DURATION.with_label_values(&["/test", "GET"]).observe(0.5);
        let duration = HTTP_DURATION.with_label_values(&["/test", "GET"]).get_sample_sum();
        assert!(duration > 0.4 && duration < 0.6);
    }

    #[test]
    fn test_system_metrics() {
        MEMORY_USAGE.set(1024.0);
        assert_eq!(MEMORY_USAGE.get(), 1024.0);

        CPU_USAGE.set(50.0);
        assert_eq!(CPU_USAGE.get(), 50.0);

        GUILD_COUNT.set(10);
        assert_eq!(GUILD_COUNT.get(), 10);

        MEMBER_COUNT.set(100);
        assert_eq!(MEMBER_COUNT.get(), 100);
    }

    #[test]
    fn test_interaction_metrics() {
        INTERACTION_REQUESTS.with_label_values(&["test"]).inc();
        assert_eq!(INTERACTION_REQUESTS.with_label_values(&["test"]).get(), 1);

        INTERACTION_ERRORS.with_label_values(&["test"]).inc();
        assert_eq!(INTERACTION_ERRORS.with_label_values(&["test"]).get(), 1);

        INTERACTION_DURATION.with_label_values(&["test"]).observe(0.5);
        let duration = INTERACTION_DURATION.with_label_values(&["test"]).get_sample_sum();
        assert!(duration > 0.4 && duration < 0.6);
    }
}
