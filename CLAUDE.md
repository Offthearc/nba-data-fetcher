# nba-data-fetcher

## Build
```bash
cargo build
cargo build --release
```

## Run
```bash
cargo run -- --help
cargo run              # today's games
cargo run -- -y        # yesterday's games
cargo run -- -d 2025-01-15  # specific date
```

## Test
```bash
cargo test
```

## Lint / Format
```bash
cargo clippy -- -D warnings
cargo fmt --check
cargo fmt
```

## Health Check
```bash
bash init.sh
```

## Rules
- No `unwrap()` in production code — use `?` or explicit error handling
- All async functions must be `async fn` with `tokio` runtime
- Run `cargo clippy` and `cargo fmt` before committing
- Use `anyhow` for error propagation in binary crate
- ESPN API base: `https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard`
- Date query param format: `?dates=YYYYMMDD`
