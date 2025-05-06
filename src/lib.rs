pub mod commands;
pub mod db;
pub mod interactions;
pub mod metrics;
pub mod schema;
pub mod utils;

pub use crate::db::Pool;
pub use crate::metrics::{
    command::{
        COUNTER as COMMAND_COUNTER, DURATION as COMMAND_DURATION, FAILURES as COMMAND_FAILURES,
    },
    discord::{CHANNEL_COUNT, GUILD_COUNT, USER_COUNT},
    guild::{
        AFK_TIMEOUT, BOOST_COUNT, BOT_COUNT, CATEGORY_CHANNEL_COUNT,
        CHANNEL_COUNT as GUILD_CHANNEL_COUNT, CREATION_TIME, EMOJI_COUNT, HUMAN_COUNT,
        MEMBER_COUNT, ONLINE_COUNT, OWNER_ID, PREMIUM_TIER, ROLE_COUNT, STICKER_COUNT,
        TEXT_CHANNEL_COUNT, VOICE_CHANNEL_COUNT,
    },
    http::{DURATION as HTTP_DURATION, REQUESTS as HTTP_REQUESTS},
    interaction::{
        AUTOCOMPLETE_REQUESTS, BUTTON_CLICKS, CONTEXT_MENU_USAGE, MODAL_SUBMISSIONS,
        SELECT_MENU_USAGE, SLASH_COMMAND_USAGE,
    },
    process::{CPU_USAGE, DB_POOL_CONNECTIONS, MEMORY_USAGE, START_TIME as PROCESS_START_TIME},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

use crate::interactions::InteractionTracker;
use serenity::model::id::{ChannelId, GuildId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
