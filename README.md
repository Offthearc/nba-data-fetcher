# nba-data-fetcher

A Rust CLI tool to fetch NBA game data for today or yesterday and display a summary of each game.

## Prerequisites

- Rust 1.75+ (install via [rustup.rs](https://rustup.rs))
- Internet connection (uses ESPN public API — no API key required)

## Setup

```bash
git clone git@github.com:Offthearc/nba-data-fetcher.git
cd nba-data-fetcher
cargo build
```

## Usage

```bash
# Today's games
cargo run

# Yesterday's games
cargo run -- --yesterday
cargo run -- -y

# Games on a specific date
cargo run -- --date 2025-01-15
cargo run -- -d 2025-01-15
```

Or run the compiled binary directly after `cargo build --release`:

```bash
./target/release/nba-data-fetcher -y
```

## Tests

```bash
cargo test
```

## Lint & Format

```bash
cargo clippy -- -D warnings
cargo fmt
```
