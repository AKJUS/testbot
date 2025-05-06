use lazy_static::lazy_static;
use prometheus::{IntCounterVec, IntGauge, HistogramVec, register_int_counter_vec, register_histogram_vec, register_int_gauge};

lazy_static! {
    pub static ref HTTP_REQUESTS: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests made",
        &["endpoint", "method"]
    ).unwrap();

    pub static ref COMMAND_EXECUTIONS: IntCounterVec = register_int_counter_vec!(
        "command_executions_total",
        "Total number of commands executed",
        &["command"]
    ).unwrap();

    pub static ref COMMAND_DURATION: HistogramVec = register_histogram_vec!(
        "command_duration_seconds",
        "Time spent executing commands",
        &["command"],
        vec![0.1, 0.5, 1.0, 2.0, 5.0]
    ).unwrap();

    pub static ref GUILD_COUNT: IntGauge = register_int_gauge!(
        "guild_count",
        "Total number of guilds the bot is in"
    ).unwrap();

    pub static ref MEMBER_COUNT: IntGauge = register_int_gauge!(
        "member_count",
        "Total number of members across all guilds"
    ).unwrap();
}

/// Record an HTTP request
pub fn record_http_request(endpoint: &str, method: &str) {
    HTTP_REQUESTS.with_label_values(&[endpoint, method]).inc();
}

/// Record a command execution
pub fn record_command_execution(command: &str) {
    COMMAND_EXECUTIONS.with_label_values(&[command]).inc();
}

/// Record command execution duration
pub fn record_command_duration(command: &str, duration: f64) {
    COMMAND_DURATION.with_label_values(&[command]).observe(duration);
}

/// Update guild count
pub fn update_guild_count(count: i64) {
    GUILD_COUNT.set(count);
}

/// Update member count
pub fn update_member_count(count: i64) {
    MEMBER_COUNT.set(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        // Test HTTP request metrics
        record_http_request("/test", "GET");
        assert_eq!(HTTP_REQUESTS.with_label_values(&["/test", "GET"]).get(), 1);

        // Test command execution metrics
        record_command_execution("test_command");
        assert_eq!(COMMAND_EXECUTIONS.with_label_values(&["test_command"]).get(), 1);

        // Test command duration metrics
        record_command_duration("test_command", 1.5);
        let duration = COMMAND_DURATION.with_label_values(&["test_command"]).get_sample_sum();
        assert!(duration > 1.4 && duration < 1.6);

        // Test guild count metrics
        update_guild_count(5);
        assert_eq!(GUILD_COUNT.get(), 5);

        // Test member count metrics
        update_member_count(100);
        assert_eq!(MEMBER_COUNT.get(), 100);
    }
} 