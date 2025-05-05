use crate::utils::random_choice;

pub(crate) static RESPONSES: [&str; 41] = [
    "Pizza",
    "Burgers",
    "Sandwiches",
    "Fried Chicken",
    "Sushi",
    "Chinese",
    "Indian",
    "Thai",
    "Mexican",
    "Italian",
    "Mediterranean",
    "BBQ",
    "Seafood",
    "Salad",
    "Breakfast",
    "Brunch",
    "Dessert",
    "Coffee",
    "Fast Food",
    "Deli",
    "Noodles",
    "Rice Bowls",
    "Tacos",
    "Burritos",
    "Wings",
    "Pasta",
    "Soup",
    "Steak",
    "Vegetarian",
    "Vegan",
    "Wraps",
    "Subs",
    "Bakery",
    "Ice Cream",
    "Smoothies",
    "Donuts",
    "Bagels",
    "Kebabs",
    "Gyros",
    "Pho",
    "Ramen",
];

#[poise::command(slash_command, prefix_command)]
pub async fn food(ctx: poise::Context<'_, crate::Data, crate::Error>) -> Result<(), crate::Error> {
    let item = random_choice(&RESPONSES).unwrap();
    ctx.say(*item).await?;
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
    fn test_responses_contains_pizza() {
        assert!(RESPONSES.iter().any(|&s| s == "Pizza"));
    }
}
