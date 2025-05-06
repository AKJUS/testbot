use crate::models::{Description, NewDescription};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use poise::Context;

type PgPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_conn(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Failed to get DB connection from pool")
}

/// Set a key-value pair in the bot's database.
/// Usage: /set foo bar
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

/// Get the value for a key from the bot's database.
/// Usage: /get foo
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

#[cfg(test)]
mod tests {
    use crate::models::{Description, NewDescription};

    #[test]
    fn test_new_description_fields() {
        let new_desc = NewDescription {
            key: "foo",
            value: "bar",
        };
        assert_eq!(new_desc.key, "foo");
        assert_eq!(new_desc.value, "bar");
    }

    #[test]
    fn test_description_fields() {
        let desc = Description {
            key: "foo".to_string(),
            value: "bar".to_string(),
        };
        assert_eq!(desc.key, "foo");
        assert_eq!(desc.value, "bar");
    }
}
