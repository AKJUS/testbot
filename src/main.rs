mod commands;
mod models;
mod schema;
mod utils;

// #[macro_use]
// extern crate diesel_migrations;

// #[macro_use]
// extern crate diesel;
// use diesel::pg::Pg;
// use diesel::r2d2::ManageConnection;
use dotenvy::dotenv;
use poise::{self, serenity_prelude::{ClientBuilder, GatewayIntents}};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;
// use std::error::Error;

use commands::{
    advice::advice,
    ball::ball,
    botsnack::botsnack,
    desc::{set, get},
    drink::drink,
    food::food,
    github::github,
    owner::quit,
    pingpong::{ping, fart},
    random::random,
    stonks::{stonks, stonkcomp, graph},
};

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Poise user data and error type
type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// --- Poise bot entry point ---
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN")?;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder().build(manager).expect("Failed to create pool.");
    let options = poise::FrameworkOptions {
        commands: vec![
            advice(),
            ball(),
            botsnack(),
            set(),
            get(),
            drink(),
            food(),
            github(),
            quit(),
            ping(),
            fart(),
            random(),
            stonks(),
            stonkcomp(),
            graph(),
        ],
        ..Default::default()
    };
    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |_ctx, _ready, _framework| {
            let db_pool = db_pool.clone();
            Box::pin(async move { Ok(Data { db_pool }) })
        })
        .build();
    let mut client = ClientBuilder::new(token, GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}

/*
// --- Old serenity::framework::standard main logic ---
// #[tokio::main]
// #[instrument]
// async fn main() {
//     ...
// }
*/
