use serde::Deserialize;

#[derive(Deserialize)]
struct Slip {
    id: i32,
    advice: String,
}

#[derive(Deserialize)]
struct Advice {
    slip: Slip,
}

/// Get a random piece of advice.
/// Usage: /advice
#[poise::command(slash_command, prefix_command)]
pub async fn advice(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
) -> Result<(), crate::Error> {
    const ENDPOINT: &str = "https://api.adviceslip.com/advice";
    let advice = reqwest::get(ENDPOINT).await?.json::<Advice>().await?;
    let results = format!("{} - #{}", advice.slip.advice, advice.slip.id);
    ctx.say(results).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_advice_deserialize() {
        let data = json!({
            "slip": {
                "id": 123,
                "advice": "Test advice."
            }
        });
        let advice: Advice = serde_json::from_value(data).unwrap();
        assert_eq!(advice.slip.id, 123);
        assert_eq!(advice.slip.advice, "Test advice.");
    }
}
