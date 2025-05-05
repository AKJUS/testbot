#[poise::command(slash_command, prefix_command)]
pub async fn github(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let github = "https://github.com/tmcarr/testbot";
    ctx.say(format!("My code is at: {}", github)).await?;
    Ok(())
}
