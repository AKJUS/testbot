use crate::Data;
use poise::serenity_prelude::{
    ChannelId, ChannelType, Guild, GuildChannel, GuildId, Member, PremiumTier, UserId,
    model::guild::{OnlineStatus},
    serenity::model::user::OnlineStatus as SerenityOnlineStatus,
    serenity::model::gateway::Presence,
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

/// Get the number of members in a guild
pub fn count_members<F>(guild: &Guild, predicate: F) -> i64
where
    F: Fn(&Guild) -> bool,
{
    guild.members.len() as i64
}

/// Get the number of online members in a guild
pub fn count_online_members(guild: &Guild) -> i64 {
    count_members(guild, |guild| {
        guild.presences.values().any(|presence| presence.status == OnlineStatus::Online)
    })
}

/// Get the number of text channels in a guild
pub fn count_text_channels(guild: &Guild) -> i64 {
    guild.channels.values().filter(|channel| channel.kind == ChannelType::Text).count() as i64
}

/// Get the number of voice channels in a guild
pub fn count_voice_channels(guild: &Guild) -> i64 {
    guild.channels.values().filter(|channel| channel.kind == ChannelType::Voice).count() as i64
}

/// Get the number of categories in a guild
pub fn count_categories(guild: &Guild) -> i64 {
    guild.channels.values().filter(|channel| channel.kind == ChannelType::Category).count() as i64
}

/// Get the number of roles in a guild
pub fn count_roles(guild: &Guild) -> i64 {
    guild.roles.len() as i64
}

/// Get the number of emojis in a guild
pub fn count_emojis(guild: &Guild) -> i64 {
    guild.emojis.len() as i64
}

/// Get the number of boosters in a guild
pub fn count_boosters(guild: &Guild) -> i64 {
    guild.members.values().filter(|member| member.premium_since.is_some()).count() as i64
}

/// Get the boost level of a guild
pub fn get_boost_level(guild: &Guild) -> i64 {
    match guild.premium_tier {
        PremiumTier::Tier0 => 0,
        PremiumTier::Tier1 => 1,
        PremiumTier::Tier2 => 2,
        PremiumTier::Tier3 => 3,
    }
}

/// Get the number of idle members in a guild
pub fn count_idle_members(guild: &Guild) -> i64 {
    count_members(guild, |guild| {
        guild.presences.values().any(|presence| presence.status == OnlineStatus::Idle)
    })
}

/// Get the number of dnd members in a guild
pub fn count_dnd_members(guild: &Guild) -> i64 {
    count_members(guild, |guild| {
        guild.presences.values().any(|presence| presence.status == OnlineStatus::DoNotDisturb)
    })
}

/// Get the number of offline members in a guild
pub fn count_offline_members(guild: &Guild) -> i64 {
    count_members(guild, |guild| {
        guild.presences.values().any(|presence| presence.status == OnlineStatus::Offline)
    })
}

/// Get the boost level of a guild
pub fn get_boost_level_from_tier(tier: PremiumTier) -> i64 {
    match tier {
        PremiumTier::Tier0 => 0,
        PremiumTier::Tier1 => 1,
        PremiumTier::Tier2 => 2,
        PremiumTier::Tier3 => 3,
        _ => 0, // Handle any future tiers
    }
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

pub fn get_premium_tier_level(guild: &Guild) -> i64 {
    match guild.premium_tier {
        PremiumTier::None => 0,
        PremiumTier::Tier1 => 1,
        PremiumTier::Tier2 => 2,
        PremiumTier::Tier3 => 3,
        PremiumTier::Tier0 => 0,
        _ => 0, // Handle any future tiers
    }
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
            region: "us-east".to_string(),
            afk_timeout: None,
            embed_enabled: None,
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
            max_stage_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            nsfw_level: Default::default(),
            premium_progress_bar_enabled: false,
            safety_alerts_channel_id: None,
            channels: HashMap::new(),
            members: HashMap::new(),
            presences: HashMap::new(),
            voice_states: HashMap::new(),
            unavailable: false,
            member_count: 0,
            large: false,
            widget_enabled: None,
            widget_channel_id: None,
            permissions: None,
            icon_hash: None,
            owner: false,
            joined_at: None,
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
            guild_id: GuildId::new(1),
            position: 0,
            name: "text".to_string(),
            nsfw: false,
            permission_overwrites: vec![],
            parent_id: None,
            rate_limit_per_user: None,
            topic: None,
            last_message_id: None,
            bitrate: None,
            user_limit: None,
            rtc_region: None,
            video_quality_mode: None,
            message_count: None,
            member_count: None,
            default_auto_archive_duration: None,
            permissions: None,
            flags: None,
            total_message_sent: None,
            available_tags: vec![],
            applied_tags: vec![],
            default_reaction_emoji: None,
            default_thread_rate_limit_per_user: None,
            default_sort_order: None,
            default_forum_layout: None,
        };
        let voice_channel = GuildChannel {
            id: ChannelId::new(2),
            kind: ChannelType::Voice,
            guild_id: GuildId::new(1),
            position: 0,
            name: "voice".to_string(),
            nsfw: false,
            permission_overwrites: vec![],
            parent_id: None,
            rate_limit_per_user: None,
            topic: None,
            last_message_id: None,
            bitrate: None,
            user_limit: None,
            rtc_region: None,
            video_quality_mode: None,
            message_count: None,
            member_count: None,
            default_auto_archive_duration: None,
            permissions: None,
            flags: None,
            total_message_sent: None,
            available_tags: vec![],
            applied_tags: vec![],
            default_reaction_emoji: None,
            default_thread_rate_limit_per_user: None,
            default_sort_order: None,
            default_forum_layout: None,
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
        assert_eq!(count_online_members(&guild), 1);
        assert_eq!(count_text_channels(&guild), 1);
        assert_eq!(count_voice_channels(&guild), 1);

        let mut guilds = HashMap::new();
        guilds.insert(guild.id, guild);

        assert_eq!(get_guild_count(&guilds), 1);
        assert_eq!(get_user_count(&guilds), 2);
        assert_eq!(get_channel_count(&guilds), 2);
    }

    #[test]
    fn test_get_channel_count() {
        let mut channels = HashMap::new();
        let text_channel = GuildChannel::default();
        channels.insert(ChannelId::new(1), text_channel);

        let guild = Guild {
            id: GuildId::new(1),
            name: "Test Guild".to_string(),
            icon: None,
            splash: None,
            discovery_splash: None,
            owner_id: UserId::new(1),
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
            max_members: Some(100),
            vanity_url_code: None,
            description: None,
            banner: None,
            premium_tier: PremiumTier::None,
            premium_subscription_count: Some(0),
            preferred_locale: "en-US".to_string(),
            public_updates_channel_id: None,
            max_video_channel_users: None,
            max_stage_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            nsfw_level: Default::default(),
            premium_progress_bar_enabled: false,
            channels,
            members: HashMap::new(),
            presences: HashMap::new(),
            voice_states: HashMap::new(),
            unavailable: false,
            member_count: 0,
            large: false,
            widget_enabled: None,
            widget_channel_id: None,
            welcome_screen: None,
            stage_instances: vec![],
            stickers: HashMap::new(),
            threads: HashMap::new(),
            scheduled_events: HashMap::new(),
        };

        assert_eq!(get_channel_count(&guild), 1);
    }
}
