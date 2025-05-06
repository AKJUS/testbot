pub mod commands;
pub mod db;
pub mod interactions;
pub mod metrics;
pub mod models;
pub mod schema;
pub mod utils;

pub use db::Pool;
pub use interactions::InteractionTracker;

pub use crate::metrics::{
    COMMAND_REQUESTS,
    COMMAND_ERRORS,
    COMMAND_DURATION,
    HTTP_REQUESTS,
    HTTP_DURATION,
    CPU_USAGE,
    MEMORY_USAGE,
    DB_POOL_CONNECTIONS,
    PROCESS_START_TIME,
    GUILD_COUNT,
    USER_COUNT,
    CHANNEL_COUNT,
    MEMBER_COUNT,
    TEXT_CHANNEL_COUNT,
    VOICE_CHANNEL_COUNT,
    CATEGORY_COUNT,
    ROLE_COUNT,
    EMOJI_COUNT,
    BOOST_COUNT,
    BOOST_LEVEL,
    INTERACTION_REQUESTS,
    INTERACTION_ERRORS,
    INTERACTION_DURATION,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

use serenity::model::id::{ChannelId, GuildId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::time::Instant;
use poise::serenity_prelude::{
    Channel, Guild, User,
    model::channel::Channel as SerenityChannel,
    model::guild::Guild as SerenityGuild,
    model::user::User as SerenityUser,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Data {
    pub db_pool: Arc<DbPool>,
    pub command_timers: HashMap<String, Instant>,
    pub guilds: Arc<HashMap<GuildId, SerenityGuild>>,
    pub users: Arc<HashMap<UserId, SerenityUser>>,
    pub channels: Arc<HashMap<GuildId, Vec<SerenityChannel>>>,
    pub interaction_tracker: RwLock<InteractionTracker>,
}

impl Data {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
            command_timers: HashMap::new(),
            guilds: Arc::new(HashMap::new()),
            users: Arc::new(HashMap::new()),
            channels: Arc::new(HashMap::new()),
            interaction_tracker: RwLock::new(InteractionTracker::new()),
        }
    }
}
