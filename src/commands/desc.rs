use crate::models::{Description, NewDescription};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use poise::Context;

type PgPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_conn(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Failed to get DB connection from pool")
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_, crate::Data, crate::Error>,
    key: String,
    value: String,
) -> Result<(), crate::Error> {
    let pool = &ctx.data().db_pool;
    let mut conn = get_conn(pool);
    let new_desc = NewDescription {
        key: &key,
        value: &value,
    };
    diesel::insert_into(crate::schema::descriptions::table)
        .values(&new_desc)
        .on_conflict(crate::schema::descriptions::key)
        .do_update()
        .set(&new_desc)
        .execute(&mut conn)?;
    ctx.say(format!("Set {} = {}", key, value)).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get(
    ctx: Context<'_, crate::Data, crate::Error>,
    key: String,
) -> Result<(), crate::Error> {
    let pool = &ctx.data().db_pool;
    let mut conn = get_conn(pool);
    let result = crate::schema::descriptions::table
        .filter(crate::schema::descriptions::key.eq(&key))
        .first::<Description>(&mut conn)
        .optional()?;
    match result {
        Some(desc) => {
            ctx.say(format!("{} = {}", desc.key, desc.value)).await?;
        }
        None => {
            ctx.say(format!("No value found for key '{}'.", key))
                .await?;
        }
    }
    Ok(())
}
