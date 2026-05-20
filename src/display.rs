use chrono::NaiveDate;
use colored::*;

use crate::models::{GameStatus, GameSummary};

pub fn print_games(games: &[GameSummary], date: &NaiveDate) {
    let game_count = games.len();
    println!(
        "\n{}",
        format!(
            " NBA Games — {}  ({} game{}) ",
            date.format("%B %d, %Y"),
            game_count,
            if game_count == 1 { "" } else { "s" }
        )
        .bold()
        .white()
        .on_blue()
    );
    println!();

    for game in games {
        print_game(game);
        println!();
    }
}

fn print_game(game: &GameSummary) {
    let status_label = format_status(&game.status);
    let (away_score, home_score) = format_scores(game);

    // Determine winner for highlighting
    let (away_bold, home_bold) = match &game.status {
        GameStatus::Final => {
            let aw = game.away_score.unwrap_or(0);
            let hw = game.home_score.unwrap_or(0);
            (aw > hw, hw > aw)
        }
        _ => (false, false),
    };

    // Away team row
    let away_name = format!(
        "{:<25}",
        format!(
            "{} {}",
            game.away_abbrev,
            game.away_record
                .as_deref()
                .map(|r| format!("({})", r))
                .unwrap_or_default()
        )
    );
    if away_bold {
        print!("  {} {}", away_name.bold(), away_score.bold().yellow());
    } else {
        print!("  {} {}", away_name.dimmed(), away_score);
    }

    // Status in the middle (print on same line as first team, then newline)
    println!("  {}", status_label);

    // Home team row
    let home_name = format!(
        "{:<25}",
        format!(
            "{} {}",
            game.home_abbrev,
            game.home_record
                .as_deref()
                .map(|r| format!("({})", r))
                .unwrap_or_default()
        )
    );
    if home_bold {
        println!("  {} {}", home_name.bold(), home_score.bold().yellow());
    } else {
        println!("  {} {}", home_name.dimmed(), home_score);
    }

    // Full team names + venue + tip-off time
    let matchup = format!("{} @ {}", game.away_team, game.home_team);
    let venue_str = game
        .venue
        .as_deref()
        .map(|v| format!(" · {}", v))
        .unwrap_or_default();
    let time_str = match &game.status {
        GameStatus::Scheduled => format!(" · {}", game.game_time),
        _ => String::new(),
    };
    println!(
        "  {}",
        format!("{}{}{}", matchup, venue_str, time_str).dimmed()
    );
}

fn format_status(status: &GameStatus) -> ColoredString {
    match status {
        GameStatus::Scheduled => "Scheduled".cyan(),
        GameStatus::Final => "Final".green(),
        GameStatus::InProgress { period, clock } => {
            let label = format!("Q{} {}", period, clock);
            label.yellow().bold()
        }
        GameStatus::Other(desc) => desc.as_str().white(),
    }
}

fn format_scores(game: &GameSummary) -> (String, String) {
    let away = game
        .away_score
        .map(|s| s.to_string())
        .unwrap_or_else(|| "-".to_string());
    let home = game
        .home_score
        .map(|s| s.to_string())
        .unwrap_or_else(|| "-".to_string());
    (format!("{:>4}", away), format!("{:>4}", home))
}
