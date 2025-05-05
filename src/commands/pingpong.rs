#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn fart(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    ctx.say("Thbbbbbbbbbbbbbbt.... squeak.").await?;
    Ok(())
}
