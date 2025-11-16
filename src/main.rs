mod github;
mod languages;
mod visualization;

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    #[clap(short, long)]
    username: String,
    #[clap(long, default_value_t = 100)]
    per_page: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Fetching data for GitHub user: {}", args.username);

    let client = github::Client::new()?;
    let repos = client
        .fetch_repositories(&args.username, args.per_page)
        .await
        .context("Failed to fetch repositories")?;

    println!("Found {} repositories", repos.len());

    let mut repos_with_languages = Vec::new();
    for repo in repos {
        let languages = client
            .fetch_languages(&repo.languages_url)
            .await
            .unwrap_or_else(|_| HashMap::new());
        repos_with_languages.push((repo, languages));
    }

    let language_stats = languages::calculate_language_stats_detailed(&repos_with_languages);

    std::fs::create_dir_all(format!("data/{}", args.username))?;
    let raw_data_path = format!(
        "data/{}/raw_{}.json",
        args.username,
        chrono::Utc::now().format("%Y%m%d")
    );
    std::fs::write(
        &raw_data_path,
        serde_json::to_string_pretty(&repos_with_languages)?,
    )?;
    println!("Raw data saved to {}", raw_data_path);

    let processed_data_path = format!(
        "data/{}/plotted_{}.csv",
        args.username,
        chrono::Utc::now().format("%Y%m%d")
    );
    languages::save_language_stats(&language_stats, &processed_data_path)?;
    println!("Processed data saved to {}", processed_data_path);

    let treemap_path = format!(
        "data/{}/treemap_{}.svg",
        args.username,
        chrono::Utc::now().format("%Y%m%d")
    );
    visualization::generate_treemap(&language_stats, &treemap_path)?;
    println!("Treemap saved to {}", treemap_path);

    let latest_treemap_path = format!("data/{}/treemap_latest.svg", args.username);
    std::fs::copy(&treemap_path, &latest_treemap_path)?;
    println!("Latest treemap available at {}", latest_treemap_path);

    println!("Language statistics:");
    for (language, stats) in &language_stats {
        println!(
            "{}: {:.2} bytes ({} repos)",
            language, stats.bytes, stats.repo_count
        );
    }

    Ok(())
}
