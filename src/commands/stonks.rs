// use serde::de;
// use serde::{self, Deserialize, Deserializer};
// use serenity::utils::MessageBuilder;
// use std::fmt::Display;
// use std::str::FromStr;

// use crate::AlphaVantageApiToken;

use plotters::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Cursor;

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

#[poise::command(
    slash_command, prefix_command,
    description = "Show a Finviz chart for a given ticker symbol.",
    usage = "/stonks AAPL"
)]
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

#[poise::command(
    slash_command, prefix_command,
    description = "Show a comparison chart of a ticker vs. the S&P 500.",
    usage = "/stonkcomp AAPL"
)]
pub async fn stonkcomp(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
    #[rest] tickers: String,
) -> Result<(), crate::Error> {
    for stonk in tickers.split_whitespace() {
        ctx.say(format!(
            "https://stonks.egd.pw/spcomp?symbol={}",
            stonk
        ))
        .await?;
    }
    Ok(())
}

#[poise::command(
    slash_command, prefix_command,
    description = "Generate and display a graph of a stock's daily closing prices using AlphaVantage.",
    usage = "/graph AAPL"
)]
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
    let resp = reqwest::get(&url).await?.json::<TimeSeriesDaily>().await?;
    let mut dates: Vec<_> = resp.time_series.iter().collect();
    dates.sort_by_key(|(date, _)| *date);
    let closes: Vec<f64> = dates
        .iter()
        .map(|(_, data)| data.close.parse::<f64>().unwrap_or(0.0))
        .collect();
    let date_labels: Vec<&str> = dates.iter().map(|(d, _)| d.as_str()).collect();

    // Plot to a buffer
    let mut buf = vec![];
    {
        let root = BitMapBackend::with_buffer(&mut buf, (800, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let min = closes.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = closes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("{} Closing Prices", ticker), ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..closes.len(), min..max)?;
        chart.configure_mesh()
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
        poise::CreateReply::default()
            .attachment((Cursor::new(buf), format!("{}_graph.png", ticker))),
    ).await?;
    Ok(())
}


