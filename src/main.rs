use anyhow::Result;
use chrono::{Duration, Local, NaiveDate};
use clap::Parser;

use nba_data_fetcher::api;
use nba_data_fetcher::cli::Cli;
use nba_data_fetcher::display;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let date = resolve_date(&cli)?;
    let games = api::fetch_games(&date).await?;

    if games.is_empty() {
        eprintln!("No NBA games found for {}.", date.format("%B %d, %Y"));
        std::process::exit(0);
    } else {
        display::print_games(&games, &date);
    }

    Ok(())
}

fn resolve_date(cli: &Cli) -> Result<NaiveDate> {
    if let Some(ref date_str) = cli.date {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
            anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD (e.g. 2025-01-15)")
        })?;
        return Ok(date);
    }

    let today = Local::now().date_naive();
    if cli.yesterday {
        Ok(today - Duration::days(1))
    } else {
        Ok(today)
    }
}
