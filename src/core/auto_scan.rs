use reqwest;

pub struct RepoAutoScanner;

impl RepoAutoScanner {
    pub fn new() -> Self {
        Self
    }

    /// GitHub raw code 자동 수집
    pub async fn fetch(&self, repo_url: &str) -> Result<String, String> {
        let raw = format!("{}/archive/refs/heads/main.zip", repo_url);

        let body = reqwest::blocking::get(&raw)
            .map_err(|e| e.to_string())?
            .bytes()
            .map_err(|e| e.to_string())?;

        Ok(String::from_utf8_lossy(&body).to_string())
    }
}
