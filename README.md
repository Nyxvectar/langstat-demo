# langstat-rs

A Rust-based tool for analyzing GitHub repository language statistics with automated visualization.

This project is a reimplementation of the original [langstat](https://github.com/mingcheng/langstat) project written in R, but now in Rust for better performance and easier deployment.

## Features

- Fetches repository data via GitHub API
- Calculates weighted language distribution based on:
  - Repository update recency
  - Stars and forks count
  - Repository count per language
- Generates multiple visualizations, both in SVG and PNG formats:
  - Pie Charts
  - Treemaps
- Automated monthly updates via GitHub Actions (coming soon)

## Installation

To build and run this tool, you need to have Rust installed. If you don't have Rust installed yet, you can install it using [rustup](https://rustup.rs/):

```bash
rustup install stable
cargo build --release
```

## Usage

```bash
# Set environment variables (optional, but recommended for higher rate limits)
export GITHUB_TOKEN="your_github_token"

# Run analysis for a specific user
./target/release/langstat -u your_username

# Or with custom page size
./target/release/langstat -u your_username --per-page 50
```

## Requirements

- Rust >= 1.60

## Configuration

| Environment Variable | Description |
| --- | --- |
| GITHUB_TOKEN | GitHub personal access token (recommended for higher rate limits) |

## License

This project is licensed under the Apache License, Version 2.0 - see the LICENSE file for details.