# Stellaria

Stellaria is an ergonomic Rust client library for NASA's Web APIs. It provides a convenient, type-safe way to concurrently call the endpoints, build state-guaranteed request parameters, and handle the REST API gracefully.

[![Latest Version](https://img.shields.io/crates/v/stellaria.svg)](https://crates.io/crates/stellaria)
[![Build](https://github.com/aravindakshabalaji/stellaria/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/aravindakshabalaji/stellaria/actions/workflows/test.yml)
[![Documentation](https://docs.rs/stellaria/badge.svg)](https://docs.rs/stellaria)
[![License](https://img.shields.io/github/license/aravindakshabalaji/stellaria.svg)](LICENSE)

## Quick start

Add the crate to your project using `cargo add stellaria`.

> This crate uses `reqwest` for HTTP. Your binary or test harness should use `tokio` (see examples below).

## Usage

### Example: get today's APOD.

```rust
use stellaria::StellariaClient;
use stellaria::apod::ApodParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Provide your API token (or load from env)
    let token = std::env::var("API_TOKEN")?;
    let client = StellariaClient::new(token);

    // Use the builder (defaults to today's date if no range provided)
    let params = ApodParams::builder().build()?;

    let resp = client.apod.get(params).await?;
    for item in resp {
        println!("{} — {}", item.date, item.title);
    }

    Ok(())
}
```

### Example: request a date range or a count.

```rust
use chrono::NaiveDate;
use stellaria::StellariaClient;
use stellaria::apod::ApodParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("API_TOKEN")?;
    let client = StellariaClient::new(token);

    // request a specific date
    let date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();
    let params = ApodParams::builder().date(date).build()?;
    let resp = client.apod.get(params).await?;

    // request a range
    let start = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 01, 31).unwrap();
    let params = ApodParams::builder().date_range(start, end).build()?;
    let resp = client.apod.get(params).await?;

    // request a count (random selection)
    let params = ApodParams::builder().count(5).build()?;
    let resp = client.apod.get(params).await?;

    Ok(())
}
```

## Tests

The repository includes a comprehensive set of unit tests for the parameter builder and integration-style tests that hit the real API. To run tests locally, set `API_TOKEN` then:

```bash
cargo test
```

> Note: tests that contact the real API depend on network access and a valid API key.

### Environment

The test suites expect an `API_TOKEN` environment variable (or `.env` file) containing a valid NASA API key. You can obtain a key at [https://api.nasa.gov](https://api.nasa.gov).

```bash
export API_TOKEN=YOUR_NASA_API_KEY
# or use a .env file for tests
```

## Contributing

Contributions are welcome:

1. Fork the repository.
2. Create a feature branch.
3. Open a pull request with a clear description and tests.

Please run `cargo fmt` and `cargo clippy` before opening PRs.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
