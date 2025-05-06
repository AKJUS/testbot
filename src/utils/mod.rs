pub mod command;
pub mod guild;
pub mod metrics;
pub mod random;
pub mod system;
pub mod time;

use crate::Data;
use poise::serenity_prelude::{
    model::channel::{Channel, ChannelType},
    model::guild::Guild,
    model::id::{GuildId, UserId},
    model::user::{OnlineStatus, User},
};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use sysinfo::{System, SystemExt};

/// Convert a Duration to a human-readable string
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}

/// Get the owner ID of a guild
pub fn get_owner(guild: &Guild) -> UserId {
    guild.owner_id
}

/// Get the AFK timeout of a guild in seconds
pub fn get_afk_timeout(guild: &Guild) -> i64 {
    guild
        .afk_metadata
        .as_ref()
        .map_or(0, |m| i64::from(u16::from(m.afk_timeout)))
}

/// Get the creation time of a guild
pub fn get_guild_creation_time(guild: &Guild) -> i64 {
    guild.id.created_at().timestamp()
}

/// Get the number of human members in a guild
pub fn count_members<F>(guild: &Guild, predicate: F) -> i64
where
    F: Fn(&poise::serenity_prelude::Member) -> bool,
{
    guild
        .members
        .values()
        .filter(|member| predicate(member))
        .count() as i64
}

/// Get the number of bot members in a guild
pub fn count_bots(guild: &Guild) -> i64 {
    count_members(guild, |member| member.user.bot)
}

/// Get the number of channels in a guild
pub fn get_guild_channel_count(guild: &Guild) -> i64 {
    guild.channels.len() as i64
}

/// Get the number of guilds the bot is in
pub fn get_guild_count(data: &Data) -> i64 {
    data.guilds.len() as i64
}

/// Get the number of users visible to the bot
pub fn get_user_count(data: &Data) -> i64 {
    data.users.len() as i64
}

/// Get the number of channels visible to the bot
pub fn get_channel_count(data: &Data) -> i64 {
    data.channels.values().map(|v| v.len()).sum::<usize>() as i64
}

/// Get the number of DB pool connections
pub fn get_db_pool_connections(data: &Data) -> i64 {
    data.db_pool.state().connections as i64
}

/// Get the memory usage of the bot process in bytes
pub fn get_memory_usage() -> i64 {
    let mut sys = System::new_all();
    sys.refresh_memory();
    sys.used_memory() as i64
}

/// Get the CPU usage of the bot process in percent
pub fn get_cpu_usage() -> i64 {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    sys.global_cpu_usage() as i64
}

/// Get the process start time in seconds since epoch
pub fn get_process_start_time() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Get the number of online members in a guild
pub fn get_online_members(guild: &Guild) -> i64 {
    guild.presences.iter()
        .filter(|(_, presence)| matches!(presence.status, OnlineStatus::Online))
        .count() as i64
}

/// Get the premium tier level of a guild
pub fn get_premium_tier_level(guild: &Guild) -> i64 {
    match guild.premium_tier {
        poise::serenity_prelude::PremiumTier::Tier0 => 0,
        poise::serenity_prelude::PremiumTier::Tier1 => 1,
        poise::serenity_prelude::PremiumTier::Tier2 => 2,
        poise::serenity_prelude::PremiumTier::Tier3 => 3,
        _ => 0, // Handle any future tiers
    }
}

/// Get the number of roles in a guild
pub fn get_guild_role_count(guild: &Guild) -> i64 {
    guild.roles.len() as i64
}

/// Get the number of members in a guild
pub fn get_guild_member_count(guild: &Guild) -> i64 {
    guild.member_count as i64
}

/// Get the number of emojis in a guild
pub fn get_guild_emoji_count(guild: &Guild) -> i64 {
    guild.emojis.len() as i64
}

/// Get the number of stickers in a guild
pub fn get_guild_sticker_count(guild: &Guild) -> i64 {
    guild.stickers.len() as i64
}

/// Get the number of boosts in a guild
pub fn get_guild_boost_count(guild: &Guild) -> i64 {
    guild.premium_subscription_count.unwrap_or(0) as i64
}

/// Get the premium tier of a guild
pub fn get_premium_tier(guild: &Guild) -> i64 {
    guild.premium_tier as i64
}

/// Get the number of channels in a guild
pub fn count_channels<F>(guild: &Guild, predicate: F) -> i64
where
    F: Fn(&Channel) -> bool,
{
    guild
        .channels
        .values()
        .filter(|channel| predicate(channel))
        .count() as i64
}

/// Get the number of text channels in a guild
pub fn get_text_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| channel.kind == ChannelType::Text)
}

/// Get the number of voice channels in a guild
pub fn get_voice_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| channel.kind == ChannelType::Voice)
}

/// Get the number of category channels in a guild
pub fn get_categories(guild: &Guild) -> i64 {
    count_channels(guild, |channel| channel.kind == ChannelType::Category)
}

// Re-export commonly used items
pub use guild::*;
pub use system::*;
pub use time::*;

pub fn random_choice<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..items.len());
    Some(&items[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use crate::interactions::InteractionTracker;

    fn create_test_pool() -> Pool<ConnectionManager<PgConnection>> {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool")
    }

    fn create_test_data() -> Data {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool");

        Data {
            db_pool: Arc::new(pool),
            command_timers: HashMap::new(),
            guilds: Arc::new(HashMap::new()),
            users: Arc::new(HashMap::new()),
            channels: Arc::new(HashMap::new()),
            interaction_tracker: RwLock::new(InteractionTracker::new(Arc::new(create_test_pool()))),
        }
    }

    #[test]
    fn test_random_choice() {
        let items = vec![1, 2, 3, 4, 5];
        let choice = random_choice(&items);
        assert!(choice.is_some());
        assert!(items.contains(choice.unwrap()));
    }

    #[test]
    fn test_random_choice_empty() {
        let items: Vec<i32> = vec![];
        let choice = random_choice(&items);
        assert!(choice.is_none());
    }

    #[test]
    fn test_guild_count() {
        let data = create_test_data();
        assert_eq!(get_guild_count(&data), 0);
    }

    #[test]
    fn test_user_count() {
        let data = create_test_data();
        assert_eq!(get_user_count(&data), 0);
    }

    #[test]
    fn test_channel_count() {
        let data = create_test_data();
        assert_eq!(get_channel_count(&data), 0);
    }

    #[test]
    fn test_db_pool_connections() {
        let data = create_test_data();
        assert_eq!(get_db_pool_connections(&data), 0);
    }
}
