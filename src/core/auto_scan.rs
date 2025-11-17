use std::process::Command;

pub struct RepoAutoScanner;

impl RepoAutoScanner {
    pub fn new() -> Self {
        Self
    }

    /// 주어진 GitHub repo URL에서 README 불러오기
    pub fn fetch(&self, repo_url: &str) -> Result<String, String> {
        let output = Command::new("curl")
            .arg("-L")
            .arg(repo_url)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
