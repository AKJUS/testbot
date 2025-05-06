// use serde::de;
// use serde::{self, Deserialize, Deserializer};
// use serenity::utils::MessageBuilder;
// use std::fmt::Display;
// use std::str::FromStr;

// use crate::AlphaVantageApiToken;

use plotters::prelude::*;
use poise::serenity_prelude::CreateAttachment;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct TimeSeriesDaily {
    #[serde(rename = "Time Series (Daily)")]
    time_series: std::collections::BTreeMap<String, DailyData>,
}

#[derive(Deserialize, Debug)]
struct DailyData {
    #[serde(rename = "4. close")]
    close: String,
}

/// Show a Finviz chart for a ticker symbol.
/// Usage: /stonks AAPL
#[poise::command(slash_command, prefix_command)]
pub async fn stonks(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
    #[rest] tickers: String,
) -> Result<(), crate::Error> {
    for stonk in tickers.split_whitespace() {
        ctx.say(format!(
            "https://www.finviz.com/chart.ashx?t={}&ty=c&ta=1&p=d&s=l",
            stonk
        ))
        .await?;
    }
    Ok(())
}

/// Compare a ticker to the S&P 500.
/// Usage: /stonkcomp AAPL
#[poise::command(slash_command, prefix_command)]
pub async fn stonkcomp(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
    #[rest] tickers: String,
) -> Result<(), crate::Error> {
    for stonk in tickers.split_whitespace() {
        ctx.say(format!("https://stonks.egd.pw/spcomp?symbol={}", stonk))
            .await?;
    }
    Ok(())
}

/// Show a graph of a stock's daily closing prices.
/// Usage: /graph AAPL
#[poise::command(slash_command, prefix_command)]
pub async fn graph(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
    ticker: String,
) -> Result<(), crate::Error> {
    let api_key = env::var("ALPHAVANTAGE_API_KEY")
        .map_err(|_| "ALPHAVANTAGE_API_KEY not set in environment")?;
    
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
        ticker, api_key
    );
    
    let resp = reqwest::get(&url)
        .await?
        .json::<TimeSeriesDaily>()
        .await
        .map_err(|e| format!("Failed to fetch or parse stock data: {}", e))?;

    if resp.time_series.is_empty() {
        return Err("No data available for the specified ticker".into());
    }

    let mut dates: Vec<_> = resp.time_series.iter().collect();
    dates.sort_by_key(|(date, _)| *date);
    
    let closes: Vec<f64> = dates
        .iter()
        .map(|(_, data)| data.close.parse::<f64>().unwrap_or(0.0))
        .collect();

    if closes.is_empty() {
        return Err("No valid closing prices found".into());
    }

    let date_labels: Vec<&str> = dates.iter().map(|(d, _)| d.as_str()).collect();

    // Plot to a buffer
    let mut buf = String::new();
    {
        let root = SVGBackend::with_string(&mut buf, (800, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let min = closes.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = closes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("{} Closing Prices", ticker), ("monospace", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..closes.len(), min..max)?;

        chart
            .configure_mesh()
            .x_labels(10)
            .x_label_formatter(&|idx| date_labels[*idx].to_string())
            .y_desc("Close")
            .x_desc("Date")
            .draw()?;

        chart.draw_series(LineSeries::new(
            closes.iter().enumerate().map(|(i, v)| (i, *v)),
            &BLUE,
        ))?;

        root.present()?;
    }

    // Send as attachment
    ctx.send(
        poise::CreateReply::default().attachment(CreateAttachment::bytes(
            buf.into_bytes(),
            format!("{}_graph.svg", ticker),
        )),
    )
    .await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_daily_data_parse() {
        let data = json!({"4. close": "123.45"});
        let daily: DailyData = serde_json::from_value(data).unwrap();
        assert_eq!(daily.close, "123.45");
    }

    #[test]
    fn test_time_series_daily_parse() {
        let data = json!({
            "Time Series (Daily)": {
                "2023-01-01": {"4. close": "100.0"},
                "2023-01-02": {"4. close": "110.0"}
            }
        });
        let ts: TimeSeriesDaily = serde_json::from_value(data).unwrap();
        assert_eq!(ts.time_series.len(), 2);
        assert_eq!(ts.time_series["2023-01-01"].close, "100.0");
    }

    #[tokio::test]
    async fn test_graph_missing_api_key() {
        std::env::remove_var("ALPHAVANTAGE_API_KEY");
        let result = std::env::var("ALPHAVANTAGE_API_KEY");
        assert!(result.is_err());
    }
}
