use crate::utils::random_choice;

pub(crate) static RESPONSES: [&str; 20] = [
    "As I see it, yes.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "It is certain.",
    "It is decidedly so.",
    "Most likely.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Outlook good.",
    "Reply hazy, try again.",
    "Signs point to yes.",
    "Very doubtful.",
    "Without a doubt.",
    "Yes.",
    "Yes – definitely.",
    "You may rely on it.",
];

#[poise::command(
    slash_command,
    prefix_command,
    description = "Ask the magic 8-ball a question.",
    usage = "/ball Will I win the lottery?"
)]
pub async fn ball(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let choice = random_choice(&RESPONSES).unwrap();
    ctx.say(*choice).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responses_not_empty() {
        assert!(!RESPONSES.is_empty());
    }

    #[test]
    fn test_responses_contains_yes() {
        assert!(RESPONSES
            .iter()
            .any(|&s| s.contains("yes") || s.contains("Yes")));
    }
}
