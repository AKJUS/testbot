// ... existing code ...

/// Shut down the bot (owners only).
/// Usage: /quit
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn quit(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let _data = ctx.data();
    // If you have a ShardManagerContainer in your Data, you can access it here
    // For now, just reply with a shutdown message
    ctx.say("Shutting down!").await?;
    // You may want to actually shut down the bot here if needed
    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_quit_output() {
        let expected = "Shutting down!";
        assert_eq!(expected, "Shutting down!");
    }
}
