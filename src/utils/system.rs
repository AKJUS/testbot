use crate::Data;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use mockall::mock;
use mockall::predicate::*;
use poise::serenity_prelude::{ChannelId, Guild, GuildChannel, GuildId, User, UserId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Process, System, SystemExt};
use tokio::sync::RwLock;

/// Error type for system operations
#[derive(Debug)]
pub enum SystemError {
    MemoryError(String),
    CpuError(String),
    TimeError(String),
    DatabaseError(String),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            SystemError::CpuError(msg) => write!(f, "CPU error: {}", msg),
            SystemError::TimeError(msg) => write!(f, "Time error: {}", msg),
            SystemError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for SystemError {}

/// Trait for collecting system metrics
pub trait SystemMetric {
    type Output;
    fn collect() -> Result<Self::Output, SystemError>;
}

/// Get the number of DB pool connections
pub fn get_db_pool_connections(data: &Data) -> Result<i64, SystemError> {
    Ok(data.db_pool.state().connections as i64)
}

/// Get the current memory usage in bytes
pub fn get_memory_usage() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.used_memory()
}

/// Get the current CPU usage as a percentage
pub fn get_cpu_usage() -> f64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.global_cpu_info().cpu_usage()
}

/// Get the process start time in seconds since UNIX epoch
pub fn get_process_start_time() -> i64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Some(process) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        process.start_time() as i64
    } else {
        0
    }
}

/// Get the uptime in seconds
pub fn get_uptime() -> Result<i64, Box<dyn Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(System::uptime() as i64)
}

/// Get all system metrics
pub fn get_all_system_metrics() -> Result<(i64, i64, i64), Box<dyn Error>> {
    Ok((
        get_memory_usage() as i64,
        get_cpu_usage() as i64,
        get_process_start_time(),
    ))
}

/// Display a system error
pub fn display_error(error: Box<dyn Error>) -> String {
    format!("System error: {}", error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        PgConnection {}
        impl PgConnection {
            fn establish(url: &str) -> Result<Self, diesel::ConnectionError>;
        }
    }

    fn create_test_data() -> Data {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("Failed to create pool");

        Data {
            db_pool: pool,
            command_timers: Arc::new(RwLock::new(HashMap::new())),
            guilds: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[test]
    fn test_system_metrics() {
        // Test memory usage
        let memory = get_memory_usage();
        assert!(memory > 0);

        // Test CPU usage
        let cpu = get_cpu_usage();
        assert!(cpu >= 0.0 && cpu <= 100.0);

        // Test process start time
        let start_time = get_process_start_time();
        assert!(start_time > 0);

        // Test getting all metrics at once
        let (mem, cpu, time) = get_all_system_metrics().unwrap();
        assert!(mem > 0);
        assert!(cpu >= 0.0 && cpu <= 100.0);
        assert!(time > 0);
    }

    #[test]
    fn test_db_pool_connections() {
        let data = create_test_data();
        let connections = get_db_pool_connections(&data).unwrap();
        assert!(connections >= 0);
    }

    #[test]
    fn test_system_error_display() {
        let memory_error = SystemError::MemoryError("test error".to_string());
        assert_eq!(memory_error.to_string(), "Memory error: test error");

        let cpu_error = SystemError::CpuError("test error".to_string());
        assert_eq!(cpu_error.to_string(), "CPU error: test error");

        let time_error = SystemError::TimeError("test error".to_string());
        assert_eq!(time_error.to_string(), "Time error: test error");

        let db_error = SystemError::DatabaseError("test error".to_string());
        assert_eq!(db_error.to_string(), "Database error: test error");
    }

    #[test]
    fn test_memory_usage() {
        let memory = get_memory_usage();
        assert!(memory > 0);
    }

    #[test]
    fn test_cpu_usage() {
        let cpu = get_cpu_usage();
        assert!(cpu >= 0.0 && cpu <= 100.0);
    }

    #[test]
    fn test_process_start_time() {
        let start_time = get_process_start_time();
        assert!(start_time > 0);
    }

    #[test]
    fn test_uptime() {
        let uptime = get_uptime().unwrap();
        assert!(uptime > 0);
    }

    #[test]
    fn test_error_display() {
        let error: Box<dyn Error> = "Test error".into();
        let display = display_error(error);
        assert_eq!(display, "System error: Test error");
    }
}
