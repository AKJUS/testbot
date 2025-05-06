use crate::utils::system::SystemError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Get the current time in seconds since epoch
pub fn get_current_time() -> Result<i64, SystemError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .map_err(|e| SystemError::TimeError(e.to_string()))
}

/// Get the elapsed time since a given timestamp in seconds
pub fn get_elapsed_time(start_time: i64) -> Result<i64, SystemError> {
    get_current_time().map(|now| now - start_time)
}

/// Format a duration in seconds into a human-readable string
pub fn format_duration(seconds: i64) -> String {
    const SECONDS_PER_MINUTE: i64 = 60;
    const SECONDS_PER_HOUR: i64 = 3600;
    const SECONDS_PER_DAY: i64 = 86400;

    if seconds < SECONDS_PER_MINUTE {
        format!("{}s", seconds)
    } else if seconds < SECONDS_PER_HOUR {
        format!(
            "{}m {}s",
            seconds / SECONDS_PER_MINUTE,
            seconds % SECONDS_PER_MINUTE
        )
    } else if seconds < SECONDS_PER_DAY {
        format!(
            "{}h {}m",
            seconds / SECONDS_PER_HOUR,
            (seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE
        )
    } else {
        format!(
            "{}d {}h",
            seconds / SECONDS_PER_DAY,
            (seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR
        )
    }
}

/// Format a timestamp into a human-readable string
pub fn format_timestamp(timestamp: i64) -> String {
    let duration = Duration::from_secs(timestamp as u64);
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Get the time difference between two timestamps in seconds
pub fn get_time_difference(time1: i64, time2: i64) -> i64 {
    time1 - time2
}

/// Check if a timestamp is within a given duration from now
pub fn is_within_duration(time: i64, duration: Duration) -> Result<bool, SystemError> {
    get_current_time().map(|now| {
        let diff = now - time;
        diff <= duration.as_secs() as i64
    })
}

/// Get the remaining time until a timestamp
pub fn get_remaining_time(end_time: i64) -> Result<i64, SystemError> {
    get_current_time().map(|now| if end_time > now { end_time - now } else { 0 })
}

/// Format the remaining time until a timestamp
pub fn format_remaining_time(timestamp: i64) -> Result<String, SystemError> {
    let remaining = get_remaining_time(timestamp)?;
    Ok(format_duration(remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_current_time() {
        assert!(get_current_time().is_ok());
    }

    #[test]
    fn test_elapsed_time() {
        let start = get_current_time().unwrap();
        thread::sleep(Duration::from_secs(1));
        let elapsed = get_elapsed_time(start).unwrap();
        assert!(elapsed >= 1);
    }

    #[test]
    fn test_time_difference() {
        let time1 = 1000;
        let time2 = 500;
        assert_eq!(get_time_difference(time1, time2), 500);
    }

    #[test]
    fn test_within_duration() {
        let now = get_current_time().unwrap();
        assert!(is_within_duration(now, Duration::from_secs(1)).unwrap());
        assert!(!is_within_duration(now - 2, Duration::from_secs(1)).unwrap());
    }

    #[test]
    fn test_remaining_time() {
        let now = get_current_time().unwrap();
        let end = now + 10;
        let remaining = get_remaining_time(end).unwrap();
        assert!(remaining <= 10);
        assert!(remaining >= 0);
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(30), "30s");
        assert_eq!(format_timestamp(90), "1m 30s");
        assert_eq!(format_timestamp(3661), "1h 1m 1s");
    }
}
