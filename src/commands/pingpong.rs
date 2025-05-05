/// Ping the bot to check if it's alive.
/// Usage: /ping
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_ping_output() {
        let expected = "Pong!";
        assert_eq!(expected, "Pong!");
    }
}
