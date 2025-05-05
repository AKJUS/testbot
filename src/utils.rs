use rand::seq::IteratorRandom;
use chrono::{Utc, Duration};
use sysinfo::{System, ProcessRefreshKind, ProcessesToUpdate};
use crate::metrics::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::PgConnection;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::prelude::*;
use diesel::ExpressionMethods;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::rng();
    items.iter().choose(&mut rng)
}

pub fn prune_command_history(conn: &mut PgConnection, days: i64) -> Result<usize, diesel::result::Error> {
    use crate::schema::command_history::dsl::*;
    let cutoff = Utc::now().naive_utc() - Duration::days(days);
    diesel::delete(command_history.filter(timestamp.lt(cutoff))).execute(conn)
}

pub fn set_process_metrics(pool: &Pool<ConnectionManager<PgConnection>>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    PROCESS_START_TIME.set(start);
    update_resource_metrics(pool);
}

pub fn update_resource_metrics(pool: &Pool<ConnectionManager<PgConnection>>) {
    DB_POOL_CONNECTIONS.set(pool.state().connections as i64);
    let mut sys = System::new_all();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    if let Some(proc) = sys.process(sysinfo::get_current_pid().unwrap()) {
        MEMORY_USAGE.set(proc.memory() as i64 * 1024);
        CPU_USAGE.set(proc.cpu_usage() as i64);
    }
}

pub fn update_discord_metrics(ctx: &poise::serenity_prelude::Context) {
    let guilds = ctx.cache.guilds().len();
    let users = ctx.cache.user_count();
    let channels = ctx.cache.guild_channel_count();
    DISCORD_GUILD_COUNT.set(guilds as i64);
    DISCORD_USER_COUNT.set(users as i64);
    DISCORD_CHANNEL_COUNT.set(channels as i64);
}

pub fn update_guild_metrics(ctx: &poise::serenity_prelude::Context) {
    use poise::serenity_prelude::{OnlineStatus, ChannelType};
    for guild_id in ctx.cache.guilds() {
        if let Some(guild_data) = ctx.cache.guild(guild_id) {
            let guild_id_str = guild_data.id.to_string();
            let guild_name = guild_data.name.clone();
            let member_count = guild_data.member_count as i64;
            let channel_count = guild_data.channels.len() as i64;
            let role_count = guild_data.roles.len() as i64;
            let creation_time = guild_data.id.created_at().timestamp();
            let mut online_count = 0;
            let mut human_count = 0;
            let mut bot_count = 0;
            for member in guild_data.members.values() {
                if member.user.bot {
                    bot_count += 1;
                } else {
                    human_count += 1;
                }
                if let Some(presence) = guild_data.presences.get(&member.user.id) {
                    if presence.status != OnlineStatus::Offline {
                        online_count += 1;
                    }
                }
            }
            let mut text_channels = 0;
            let mut voice_channels = 0;
            let mut category_channels = 0;
            for channel in guild_data.channels.values() {
                match channel.kind {
                    ChannelType::Text | ChannelType::News | ChannelType::PublicThread | ChannelType::PrivateThread => text_channels += 1,
                    ChannelType::Voice | ChannelType::Stage => voice_channels += 1,
                    ChannelType::Category => category_channels += 1,
                    _ => {}
                }
            }
            let emoji_count = guild_data.emojis.len() as i64;
            let sticker_count = guild_data.stickers.len() as i64;
            let boost_count = guild_data.premium_subscription_count.unwrap_or(0) as i64;
            let premium_tier = u8::from(guild_data.premium_tier) as i64;
            let afk_timeout = guild_data.afk_metadata.map(|m| m.afk_timeout.as_secs() as i64).unwrap_or(0);
            let owner_id = guild_data.owner_id.to_string();
            GUILD_MEMBER_COUNT.set(member_count);
            GUILD_CHANNEL_COUNT.set(channel_count);
            GUILD_ROLE_COUNT.set(role_count);
            GUILD_ONLINE_COUNT.set(online_count);
            GUILD_CREATION_TIME.set(creation_time);
            GUILD_HUMAN_COUNT.set(human_count);
            GUILD_BOT_COUNT.set(bot_count);
            GUILD_TEXT_CHANNEL_COUNT.set(text_channels);
            GUILD_VOICE_CHANNEL_COUNT.set(voice_channels);
            GUILD_CATEGORY_CHANNEL_COUNT.set(category_channels);
            GUILD_EMOJI_COUNT.set(emoji_count);
            GUILD_STICKER_COUNT.set(sticker_count);
            GUILD_BOOST_COUNT.set(boost_count);
            GUILD_PREMIUM_TIER.set(premium_tier);
            GUILD_OWNER_ID.set(1);
            GUILD_AFK_TIMEOUT.set(afk_timeout);
        }
    }
}

pub fn prometheus_metrics() -> String {
    use prometheus::{Encoder, TextEncoder, gather};
    let encoder = TextEncoder::new();
    let metric_families = gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_choice_empty() {
        let items: [i32; 0] = [];
        assert!(random_choice(&items).is_none());
    }

    #[test]
    fn test_random_choice_single() {
        let items = [42];
        assert_eq!(random_choice(&items), Some(&42));
    }

    #[test]
    fn test_random_choice_multiple() {
        let items = [1, 2, 3, 4, 5];
        let result = random_choice(&items);
        assert!(result.is_some());
        assert!(items.contains(result.unwrap()));
    }
}
