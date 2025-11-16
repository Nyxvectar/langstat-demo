use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub updated_at: String,
}

pub struct Client {
    client: reqwest::Client,
    token: Option<String>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let token = std::env::var("GITHUB_TOKEN").ok();
        let client = reqwest::Client::new();
        Ok(Client { client, token })
    }

    pub async fn fetch_repositories(
        &self,
        username: &str,
        per_page: u32,
    ) -> Result<Vec<Repository>> {
        let mut all_repos = Vec::new();
        let mut page = 1;

        loop {
            let url = format!(
                "https://api.github.com/users/{}/repos?per_page={}&page={}",
                username, per_page, page
            );

            let mut request = self.client.get(&url);

            if let Some(token) = &self.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            request = request.header("User-Agent", "LangStat-Rust");

            let response = request.send().await?;

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await?;
                anyhow::bail!("GitHub API request failed with status {}: {}", status, text);
            }

            let repos: Vec<Repository> = response.json().await?;

            if repos.is_empty() {
                break;
            }

            all_repos.extend(repos);
            page += 1;
        }

        Ok(all_repos)
    }

    pub async fn fetch_languages(&self, languages_url: &str) -> Result<HashMap<String, u64>> {
        let mut request = self.client.get(languages_url);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        request = request.header("User-Agent", "LangStat-Rust");

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("GitHub API request failed with status {}: {}", status, text);
        }

        let language_data: HashMap<String, u64> = response.json().await?;
        Ok(language_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_repositories() {
        let client = Client::new().unwrap();
        let repos = client.fetch_repositories("rust-lang", 100).await.unwrap();
        assert!(repos.len() > 0);
    }

    #[tokio::test]
    async fn test_fetch_languages() {
        let client = Client::new().unwrap();
    }
}
