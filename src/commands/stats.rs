use poise::Context;
use crate::models::CommandStat;
use diesel::prelude::*;
use diesel::ExpressionMethods;

/// Show command usage statistics.
/// Usage: /stats
#[poise::command(slash_command, prefix_command)]
pub async fn stats(ctx: Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let pool = &ctx.data().db_pool;
    let mut conn = pool.get().unwrap();
    use crate::schema::command_stats::dsl::*;
    let results = command_stats
        .order(count.desc())
        .limit(10)
        .load::<CommandStat>(&mut conn)
        .unwrap_or_default();

    let mut msg = String::from("Top commands:\n");
    for stat in results {
        msg.push_str(&format!(
            "{} ({}): {} times, last used {}\n",
            stat.command, stat.arguments, stat.count, stat.last_used
        ));
    }
    ctx.say(msg).await?;
    Ok(())
} 