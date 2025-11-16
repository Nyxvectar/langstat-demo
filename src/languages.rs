use crate::github::Repository;
use anyhow::Result;
use csv::Writer;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct LanguageStats {
    pub bytes: f64,
    pub repo_count: u32,
}

pub fn calculate_language_stats_detailed(
    repo_languages: &[(Repository, HashMap<String, u64>)],
) -> HashMap<String, LanguageStats> {
    let mut language_stats: HashMap<String, LanguageStats> = HashMap::new();

    for (repo, languages) in repo_languages {
        let total_bytes: u64 = languages.values().sum();
        if total_bytes == 0 {
            continue;
        }

        for (language, bytes) in languages {
            let stats = language_stats.entry(language.clone()).or_default();
            stats.repo_count += 1;

            let percentage = *bytes as f64 / total_bytes as f64;
            let weight = (repo.stargazers_count as f64 + 1.0) * (repo.forks_count as f64 + 1.0);
            stats.bytes += percentage * weight * 1000.0;
        }
    }

    language_stats
}

pub fn save_language_stats(stats: &HashMap<String, LanguageStats>, path: &str) -> Result<()> {
    let mut wtr = Writer::from_path(path)?;

    wtr.write_record(&["language", "bytes", "repo_count"])?;

    for (language, stat) in stats {
        wtr.write_record(&[
            language,
            &stat.bytes.to_string(),
            &stat.repo_count.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
