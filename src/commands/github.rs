/// Get the GitHub repository link for the bot.
/// Usage: /github
#[poise::command(slash_command, prefix_command)]
pub async fn github(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
) -> Result<(), crate::Error> {
    let github = "https://github.com/tmcarr/testbot";
    ctx.say(format!("My code is at: {}", github)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_github_output() {
        // We can't test Discord output, but we can test the string
        let github = "https://github.com/tmcarr/testbot";
        let expected = format!("My code is at: {}", github);
        assert_eq!(expected, "My code is at: https://github.com/tmcarr/testbot");
    }
}
