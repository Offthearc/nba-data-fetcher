use anyhow::{Context, Result};
use chrono::NaiveDate;
use reqwest::Client;

use crate::models::{
    Competition, Competitor, Event, GameStatus, GameSummary, ScoreboardResponse, Status, StatusType,
};

const ESPN_API_BASE: &str =
    "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard";

pub async fn fetch_games(date: &NaiveDate) -> Result<Vec<GameSummary>> {
    let date_str = date.format("%Y%m%d").to_string();

    let client = Client::builder()
        .user_agent("nba-data-fetcher/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let url = format!("{}?dates={}", ESPN_API_BASE, date_str);

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to connect to ESPN API")?;

    if !response.status().is_success() {
        anyhow::bail!("ESPN API returned status {}", response.status());
    }

    let scoreboard: ScoreboardResponse = response
        .json()
        .await
        .context("Failed to parse ESPN API response")?;

    Ok(scoreboard.events.into_iter().map(parse_event).collect())
}

fn parse_event(event: Event) -> GameSummary {
    let competition = event
        .competitions
        .into_iter()
        .next()
        .unwrap_or_else(empty_competition);

    let home = find_competitor(&competition.competitors, "home");
    let away = find_competitor(&competition.competitors, "away");

    let status = match competition.status.status_type.name.as_str() {
        "STATUS_SCHEDULED" => GameStatus::Scheduled,
        "STATUS_FINAL" | "STATUS_FULL_TIME" => GameStatus::Final,
        "STATUS_IN_PROGRESS" | "STATUS_HALFTIME" => GameStatus::InProgress {
            period: competition.status.period.unwrap_or(1),
            clock: competition.status.display_clock.unwrap_or_default(),
        },
        _ => GameStatus::Other(competition.status.status_type.description.clone()),
    };

    let venue = competition.venue.as_ref().and_then(|v| v.full_name.clone());
    let game_time = format_game_time(&event.date);

    GameSummary {
        home_team: team_name(home),
        home_abbrev: team_abbrev(home),
        home_score: parse_score(home),
        home_record: extract_record(home),
        away_team: team_name(away),
        away_abbrev: team_abbrev(away),
        away_score: parse_score(away),
        away_record: extract_record(away),
        status,
        venue,
        game_time,
    }
}

fn find_competitor<'a>(competitors: &'a [Competitor], side: &str) -> Option<&'a Competitor> {
    competitors.iter().find(|c| c.home_away == side)
}

fn team_name(c: Option<&Competitor>) -> String {
    c.map(|c| c.team.display_name.clone()).unwrap_or_default()
}

fn team_abbrev(c: Option<&Competitor>) -> String {
    c.map(|c| c.team.abbreviation.clone()).unwrap_or_default()
}

fn parse_score(c: Option<&Competitor>) -> Option<u32> {
    c.and_then(|c| c.score.as_ref())
        .and_then(|s| s.parse().ok())
}

fn extract_record(c: Option<&Competitor>) -> Option<String> {
    c.and_then(|c| c.records.as_ref())
        .and_then(|records| records.iter().find(|r| r.name == "overall"))
        .map(|r| r.summary.clone())
}

fn format_game_time(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        let local = dt.with_timezone(&chrono::Local);
        local.format("%-I:%M %p").to_string()
    } else {
        date_str.to_string()
    }
}

fn empty_competition() -> Competition {
    Competition {
        competitors: vec![],
        status: Status {
            display_clock: None,
            period: None,
            status_type: StatusType {
                name: String::new(),
                description: String::from("Unknown"),
                completed: false,
            },
        },
        venue: None,
    }
}
