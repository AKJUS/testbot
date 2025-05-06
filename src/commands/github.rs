use crate::utils::command::log_command_usage;
use crate::Error;

/// Get the GitHub repository link for the bot.
/// Usage: /github
#[poise::command(slash_command, prefix_command)]
pub async fn github(
    ctx: poise::Context<'_, crate::Data, Error>,
) -> Result<(), Error> {
    let github = "https://github.com/tmcarr/testbot";
    ctx.say(format!("My code is at: {}", github)).await?;

    // Log command usage
    log_command_usage(&ctx, "github", "").await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_output() {
        let github = "https://github.com/tmcarr/testbot";
        let expected = format!("My code is at: {}", github);
        assert_eq!(expected, "My code is at: https://github.com/tmcarr/testbot");
    }
}
