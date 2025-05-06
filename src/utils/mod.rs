pub mod command;
pub mod guild;
pub mod metrics;
pub mod system;
pub mod time;

use std::time::Duration;
use poise::serenity_prelude::{Guild, GuildId, UserId};
use crate::Data;

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
    guild.afk_metadata.as_ref().map_or(0, |m| i64::from(u16::from(m.afk_timeout)))
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
    guild.members.values()
        .filter(|member| predicate(member))
        .count() as i64
}

/// Get the number of bot members in a guild
pub fn count_bots(guild: &Guild) -> i64 {
    count_members(guild, |member| member.user.bot)
}

/// Get the number of text channels in a guild
pub fn count_text_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| matches!(channel.kind, poise::serenity_prelude::ChannelType::Text))
}

/// Get the number of voice channels in a guild
pub fn count_voice_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| matches!(channel.kind, poise::serenity_prelude::ChannelType::Voice))
}

/// Get the number of category channels in a guild
pub fn count_categories(guild: &Guild) -> i64 {
    count_channels(guild, |channel| matches!(channel.kind, poise::serenity_prelude::ChannelType::Category))
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
    match guild.premium_tier {
        poise::serenity_prelude::PremiumTier::Tier0 => 0,
        poise::serenity_prelude::PremiumTier::Tier1 => 1,
        poise::serenity_prelude::PremiumTier::Tier2 => 2,
        poise::serenity_prelude::PremiumTier::Tier3 => 3,
    }
}

/// Get the number of online members in a guild
pub fn count_online_members(guild: &Guild) -> i64 {
    count_members(guild, |member| member.user.online)
}

/// Get the number of roles in a guild
pub fn get_guild_role_count(guild: &Guild) -> i64 {
    guild.roles.len() as i64
}

/// Get the number of members in a guild
pub fn get_guild_member_count(guild: &Guild) -> i64 {
    guild.member_count as i64
}

/// Get the number of channels in a guild
pub fn get_guild_channel_count(guild: &Guild) -> i64 {
    guild.channels.len() as i64
}

/// Get the number of guilds the bot is in
pub async fn get_guild_count(data: &Data) -> i64 {
    data.guilds.read().await.len() as i64
}

/// Get the number of users visible to the bot
pub async fn get_user_count(data: &Data) -> i64 {
    data.users.read().await.len() as i64
}

/// Get the number of channels visible to the bot
pub async fn get_channel_count(data: &Data) -> i64 {
    data.channels.read().await.len() as i64
}

/// Get the number of DB pool connections
pub fn get_db_pool_connections(data: &Data) -> i64 {
    data.pool.size() as i64
}

/// Get the memory usage of the bot process in bytes
pub fn get_memory_usage() -> i64 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    sys.used_memory() as i64
}

/// Get the CPU usage of the bot process in percent
pub fn get_cpu_usage() -> i64 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_cpu();
    sys.global_cpu_info().cpu_usage() as i64
}

/// Get the process start time in seconds since epoch
pub fn get_process_start_time() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

// Re-export commonly used items
pub use guild::*;
pub use system::*;
pub use time::*;

#[cfg(test)]
mod tests {
    use super::*;
    use poise::serenity_prelude::{Guild, GuildId, UserId, Member, User, GuildChannel, ChannelType};
    use std::collections::HashMap;

    fn create_test_guild() -> Guild {
        let mut guild = Guild {
            id: GuildId::new(1),
            name: "Test Guild".to_string(),
            owner_id: UserId::new(1),
            afk_metadata: None,
            members: HashMap::new(),
            channels: HashMap::new(),
            premium_tier: poise::serenity_prelude::PremiumTier::Tier0,
            ..Default::default()
        };

        let user1 = User {
            id: UserId::new(1),
            name: "Human User".to_string(),
            bot: false,
            ..Default::default()
        };

        let user2 = User {
            id: UserId::new(2),
            name: "Bot User".to_string(),
            bot: true,
            ..Default::default()
        };

        let member1 = Member {
            user: user1,
            nick: None,
            roles: vec![],
            joined_at: Some(poise::serenity_prelude::Timestamp::now()),
            guild_id: GuildId::new(1),
            ..Default::default()
        };

        let member2 = Member {
            user: user2,
            nick: None,
            roles: vec![],
            joined_at: Some(poise::serenity_prelude::Timestamp::now()),
            guild_id: GuildId::new(1),
            ..Default::default()
        };

        guild.members.insert(UserId::new(1), member1);
        guild.members.insert(UserId::new(2), member2);

        let text_channel = GuildChannel {
            id: ChannelId::new(1),
            kind: ChannelType::Text,
            guild_id: GuildId::new(1),
            name: "text-channel".to_string(),
            position: 0,
            ..Default::default()
        };

        let voice_channel = GuildChannel {
            id: ChannelId::new(2),
            kind: ChannelType::Voice,
            guild_id: GuildId::new(1),
            name: "voice-channel".to_string(),
            position: 1,
            ..Default::default()
        };

        guild.channels.insert(ChannelId::new(1), text_channel);
        guild.channels.insert(ChannelId::new(2), voice_channel);

        guild
    }

    #[test]
    fn test_guild_metrics() {
        let guild = create_test_guild();

        assert_eq!(get_owner(&guild), UserId::new(1));
        assert_eq!(get_afk_timeout(&guild), 0);
        assert_eq!(count_members(&guild, |member| !member.user.bot), 1);
        assert_eq!(count_members(&guild, |member| member.user.bot), 1);
        assert_eq!(count_text_channels(&guild), 1);
        assert_eq!(count_voice_channels(&guild), 1);
        assert_eq!(count_categories(&guild), 0);
        assert_eq!(get_premium_tier(&guild), 0);
    }
} 