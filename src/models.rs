#![allow(dead_code)]

use serde::Deserialize;

// ESPN API response models — fields map 1:1 to API; not all are used in display

#[derive(Debug, Deserialize)]
pub struct ScoreboardResponse {
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub date: String,
    pub competitions: Vec<Competition>,
}

#[derive(Debug, Deserialize)]
pub struct Competition {
    pub competitors: Vec<Competitor>,
    pub status: Status,
    pub venue: Option<Venue>,
}

#[derive(Debug, Deserialize)]
pub struct Competitor {
    #[serde(rename = "homeAway")]
    pub home_away: String,
    pub team: Team,
    pub score: Option<String>,
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub abbreviation: String,
    pub logo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(rename = "displayClock")]
    pub display_clock: Option<String>,
    pub period: Option<u32>,
    #[serde(rename = "type")]
    pub status_type: StatusType,
}

#[derive(Debug, Deserialize)]
pub struct StatusType {
    pub name: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Venue {
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub summary: String,
}

// Internal clean model for display

#[derive(Debug)]
pub struct GameSummary {
    pub home_team: String,
    pub home_abbrev: String,
    pub home_score: Option<u32>,
    pub home_record: Option<String>,
    pub away_team: String,
    pub away_abbrev: String,
    pub away_score: Option<u32>,
    pub away_record: Option<String>,
    pub status: GameStatus,
    pub venue: Option<String>,
    pub game_time: String,
}

#[derive(Debug)]
pub enum GameStatus {
    Scheduled,
    InProgress { period: u32, clock: String },
    Final,
    Other(String),
}
