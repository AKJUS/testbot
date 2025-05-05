use rand::prelude::IteratorRandom;

#[poise::command(slash_command, prefix_command)]
pub async fn random(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
    #[rest] choices: String,
) -> Result<(), crate::Error> {
    let choices_vec: Vec<&str> = choices.split_whitespace().collect();
    let thing = choices_vec.iter().choose(&mut rand::rng());
    match thing {
        Some(choice) => ctx.say(*choice).await?,
        None => ctx.say("Why u no args?!").await?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::random_choice;

    #[test]
    fn test_choices_split() {
        let input = "foo bar baz";
        let choices_vec: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(choices_vec, vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn test_random_choice_from_choices() {
        let input = "foo bar baz";
        let choices_vec: Vec<&str> = input.split_whitespace().collect();
        let result = random_choice(&choices_vec);
        assert!(result.is_some());
        assert!(choices_vec.contains(result.unwrap()));
    }
}
