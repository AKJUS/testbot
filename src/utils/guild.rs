use crate::Data;
use poise::serenity_prelude::{
    ChannelId, ChannelType, Guild, GuildChannel, GuildId, Member, PremiumTier, UserId,
};
use std::collections::HashMap;

/// Trait for collecting metrics from guild data
pub trait GuildMetric {
    type Output;
    fn collect(guild: &Guild) -> Self::Output;
}

/// Get the owner ID of a guild
pub fn get_owner(guild: &Guild) -> UserId {
    guild.owner_id
}

/// Get the AFK timeout in seconds
pub fn get_afk_timeout(guild: &Guild) -> Option<u64> {
    guild.afk_timeout.map(|timeout| timeout.as_secs())
}

/// Get the creation time of a guild
pub fn get_creation_time(guild: &Guild) -> i64 {
    guild.id.created_at().timestamp()
}

/// Count members in a guild that match a predicate
pub fn count_members<F>(guild: &Guild, predicate: F) -> usize
where
    F: Fn(&Member) -> bool,
{
    guild.members.values().filter(|m| predicate(m)).count()
}

/// Get the number of channels in a guild by type
pub fn count_channels<F>(guild: &Guild, predicate: F) -> i64
where
    F: Fn(&GuildChannel) -> bool,
{
    guild
        .channels
        .values()
        .filter(|channel| predicate(channel))
        .count() as i64
}

/// Get the number of human members in a guild
pub fn count_humans(guild: &Guild) -> usize {
    count_members(guild, |member| !member.user.bot)
}

/// Get the number of bot members in a guild
pub fn count_bots(guild: &Guild) -> usize {
    count_members(guild, |member| member.user.bot)
}

/// Count online members in a guild
pub fn count_online_members(guild: &Guild) -> usize {
    guild
        .members
        .values()
        .filter(|m| m.presence.as_ref().map_or(false, |p| p.status.is_online()))
        .count()
}

/// Get the number of text channels in a guild
pub fn count_text_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| matches!(channel.kind, ChannelType::Text))
}

/// Get the number of voice channels in a guild
pub fn count_voice_channels(guild: &Guild) -> i64 {
    count_channels(guild, |channel| matches!(channel.kind, ChannelType::Voice))
}

/// Get the number of category channels in a guild
pub fn count_categories(guild: &Guild) -> i64 {
    count_channels(guild, |channel| {
        matches!(channel.kind, ChannelType::Category)
    })
}

/// Get the number of emojis in a guild
pub fn get_emoji_count(guild: &Guild) -> i64 {
    guild.emojis.len() as i64
}

/// Get the number of stickers in a guild
pub fn get_sticker_count(guild: &Guild) -> i64 {
    guild.stickers.len() as i64
}

/// Get the number of boosts in a guild
pub fn get_boost_count(guild: &Guild) -> i64 {
    guild.premium_subscription_count.unwrap_or(0) as i64
}

/// Get the premium tier level
pub fn get_premium_tier(guild: &Guild) -> i64 {
    match guild.premium_tier {
        PremiumTier::None => 0,
        PremiumTier::Tier1 => 1,
        PremiumTier::Tier2 => 2,
        PremiumTier::Tier3 => 3,
        PremiumTier::Other(_) => 0, // Handle any future tiers
    }
}

/// Get the number of roles in a guild
pub fn get_role_count(guild: &Guild) -> i64 {
    guild.roles.len() as i64
}

/// Get the number of members in a guild
pub fn get_member_count(guild: &Guild) -> i64 {
    guild.member_count as i64
}

/// Get the number of channels in a guild
pub fn get_channel_count(guild: &Guild) -> i64 {
    guild.channels.len() as i64
}

/// Get the total number of guilds
pub fn get_guild_count(guilds: &HashMap<GuildId, Guild>) -> usize {
    guilds.len()
}

/// Get the total number of users across all guilds
pub fn get_user_count(guilds: &HashMap<GuildId, Guild>) -> usize {
    let mut unique_users = std::collections::HashSet::new();
    for guild in guilds.values() {
        for member in guild.members.values() {
            unique_users.insert(member.user.id);
        }
    }
    unique_users.len()
}

/// Get the total number of channels across all guilds
pub fn get_channel_count(guilds: &HashMap<GuildId, Guild>) -> usize {
    guilds.values().map(|g| g.channels.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use poise::serenity_prelude::{
        ChannelId, ChannelType, GuildChannel, GuildId, Member, PremiumTier, User,
    };
    use std::collections::HashMap;

    fn create_test_guild() -> Guild {
        let mut guild = Guild {
            id: GuildId::new(1),
            name: "Test Guild".to_string(),
            icon: None,
            splash: None,
            discovery_splash: None,
            owner_id: UserId::new(1),
            afk_channel_id: None,
            afk_timeout: Some(std::time::Duration::from_secs(300)),
            verification_level: Default::default(),
            default_message_notifications: Default::default(),
            explicit_content_filter: Default::default(),
            roles: HashMap::new(),
            emojis: HashMap::new(),
            features: vec![],
            mfa_level: Default::default(),
            application_id: None,
            system_channel_id: None,
            system_channel_flags: Default::default(),
            rules_channel_id: None,
            max_presences: None,
            max_members: None,
            vanity_url_code: None,
            description: None,
            banner: None,
            premium_tier: PremiumTier::Tier1,
            premium_subscription_count: Some(2),
            preferred_locale: "en-US".to_string(),
            public_updates_channel_id: None,
            max_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            welcome_screen: None,
            nsfw_level: Default::default(),
            stage_instances: vec![],
            stickers: HashMap::new(),
            premium_progress_bar_enabled: false,
            members: HashMap::new(),
            channels: HashMap::new(),
            threads: HashMap::new(),
            presences: HashMap::new(),
            voice_states: HashMap::new(),
            application_commands: HashMap::new(),
        };

        // Add some test members
        let mut members = HashMap::new();
        let member1 = Member {
            user: User {
                id: UserId::new(1),
                bot: false,
                ..Default::default()
            },
            presence: None,
            ..Default::default()
        };
        let member2 = Member {
            user: User {
                id: UserId::new(2),
                bot: true,
                ..Default::default()
            },
            presence: None,
            ..Default::default()
        };
        members.insert(UserId::new(1), member1);
        members.insert(UserId::new(2), member2);
        guild.members = members;

        // Add some test channels
        let mut channels = HashMap::new();
        let text_channel = GuildChannel {
            id: ChannelId::new(1),
            kind: ChannelType::Text,
            ..Default::default()
        };
        let voice_channel = GuildChannel {
            id: ChannelId::new(2),
            kind: ChannelType::Voice,
            ..Default::default()
        };
        channels.insert(ChannelId::new(1), text_channel);
        channels.insert(ChannelId::new(2), voice_channel);
        guild.channels = channels;

        guild
    }

    #[test]
    fn test_guild_utilities() {
        let guild = create_test_guild();

        assert_eq!(get_afk_timeout(&guild), Some(300));
        assert_eq!(count_members(&guild, |_| true), 2);
        assert_eq!(get_premium_tier(&guild), 1);
        assert_eq!(count_humans(&guild), 1);
        assert_eq!(count_bots(&guild), 1);
        assert_eq!(count_text_channels(&guild), 1);
        assert_eq!(count_voice_channels(&guild), 1);

        let mut guilds = HashMap::new();
        guilds.insert(guild.id, guild);

        assert_eq!(get_guild_count(&guilds), 1);
        assert_eq!(get_user_count(&guilds), 2);
        assert_eq!(get_channel_count(&guilds), 2);
    }
}
