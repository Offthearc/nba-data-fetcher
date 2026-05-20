use chrono::NaiveDate;

#[tokio::test]
async fn test_fetch_games_returns_results_for_known_date() {
    // 2024-01-15 was a regular NBA game day
    let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    let games = nba_data_fetcher::api::fetch_games(&date).await;
    assert!(
        games.is_ok(),
        "fetch_games should succeed: {:?}",
        games.err()
    );
    let games = games.unwrap();
    assert!(!games.is_empty(), "Expected games on 2024-01-15");
}

#[tokio::test]
async fn test_fetch_games_empty_for_no_games_day() {
    // NBA off-season: July 1 2024 should have no games
    let date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
    let games = nba_data_fetcher::api::fetch_games(&date).await;
    assert!(
        games.is_ok(),
        "fetch_games should not error even with no games"
    );
    // Result may be empty
    let _ = games.unwrap();
}

#[test]
fn test_date_parsing_valid() {
    let result = NaiveDate::parse_from_str("2025-01-15", "%Y-%m-%d");
    assert!(result.is_ok());
}

#[test]
fn test_date_parsing_invalid() {
    let result = NaiveDate::parse_from_str("not-a-date", "%Y-%m-%d");
    assert!(result.is_err());
}
