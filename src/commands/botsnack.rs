use crate::utils::random_choice;

pub(crate) static RESPONSES: [&str; 5] = ["Yum!", "*cronch*", "MOAR", "*Smiles*", "Nice."];

#[poise::command(
    slash_command, prefix_command,
    description = "Give the bot a snack!",
    usage = "/botsnack"
)]
pub async fn botsnack(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let response = random_choice(&RESPONSES).unwrap();
    ctx.say(*response).await?;
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
    fn test_responses_contains_yum() {
        assert!(RESPONSES.iter().any(|&s| s.contains("Yum")));
    }
}
