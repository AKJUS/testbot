use crate::utils::random_choice;

pub(crate) static RESPONSES: [&str; 14] = [
    "Water.",
    "Topo Chico",
    "La Croix",
    "Water?",
    "Milk",
    "Sparking Water",
    "Seltzer Water",
    "Tap Water",
    "Voda",
    "Dihydrogen monoxide",
    "Vand",
    "Eau",
    "Akvo",
    "Agua",
];

/// Get a random drink suggestion.
/// Usage: /drink
#[poise::command(slash_command, prefix_command)]
pub async fn drink(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let drink = random_choice(&RESPONSES).unwrap();
    ctx.say(*drink).await?;
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
    fn test_responses_contains_water() {
        assert!(RESPONSES
            .iter()
            .any(|&s| s.to_lowercase().contains("water")));
    }
}
