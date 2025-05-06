pub mod commands;
pub mod db;
pub mod metrics;
pub mod schema;
pub mod utils;
pub mod interactions;

pub use crate::db::Pool;
pub use crate::metrics::{
    command::{COUNTER as COMMAND_COUNTER, FAILURES as COMMAND_FAILURES, DURATION as COMMAND_DURATION},
    http::{REQUESTS as HTTP_REQUESTS, DURATION as HTTP_DURATION},
    process::{START_TIME as PROCESS_START_TIME, DB_POOL_CONNECTIONS, MEMORY_USAGE, CPU_USAGE},
    discord::{GUILD_COUNT, USER_COUNT, CHANNEL_COUNT},
    guild::{
        MEMBER_COUNT, CHANNEL_COUNT as GUILD_CHANNEL_COUNT, ROLE_COUNT, ONLINE_COUNT,
        CREATION_TIME, HUMAN_COUNT, BOT_COUNT, TEXT_CHANNEL_COUNT, VOICE_CHANNEL_COUNT,
        CATEGORY_CHANNEL_COUNT, EMOJI_COUNT, STICKER_COUNT, BOOST_COUNT,
        PREMIUM_TIER, OWNER_ID, AFK_TIMEOUT
    },
    interaction::{
        SLASH_COMMAND_USAGE,
        BUTTON_CLICKS,
        SELECT_MENU_USAGE,
        MODAL_SUBMISSIONS,
        CONTEXT_MENU_USAGE,
        AUTOCOMPLETE_REQUESTS,
    },
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use serenity::model::id::{GuildId, UserId, ChannelId};
use crate::interactions::InteractionTracker;

pub struct Data {
    pub db_pool: Arc<db::Pool>,
    pub command_timers: Arc<RwLock<HashMap<String, u64>>>,
    pub guilds: Arc<RwLock<HashMap<GuildId, u64>>>,
    pub users: Arc<RwLock<HashMap<UserId, u64>>>,
    pub channels: Arc<RwLock<HashMap<ChannelId, u64>>>,
    pub interaction_tracker: Arc<InteractionTracker>,
}

impl Data {
    pub fn new(db_pool: db::Pool) -> Self {
        Self {
            db_pool: Arc::new(db_pool.clone()),
            command_timers: Arc::new(RwLock::new(HashMap::new())),
            guilds: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
            interaction_tracker: Arc::new(InteractionTracker::new(db_pool)),
        }
    }
} 