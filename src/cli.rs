use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "nba-data-fetcher",
    about = "Fetch and display NBA game summaries",
    long_about = "A CLI tool to fetch NBA game data for today or yesterday and display a summary of each game."
)]
pub struct Cli {
    /// Show games from yesterday instead of today
    #[arg(short, long)]
    pub yesterday: bool,

    /// Show games for a specific date (format: YYYY-MM-DD)
    #[arg(short, long, value_name = "DATE")]
    pub date: Option<String>,
}
