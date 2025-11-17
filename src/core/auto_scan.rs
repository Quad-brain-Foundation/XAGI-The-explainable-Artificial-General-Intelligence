use std::process::Command;

pub struct RepoAutoScanner;

impl RepoAutoScanner {
    pub fn new() -> Self {
        Self
    }

    /// GitHub 리포에서 코드 자동 스캔
    pub fn fetch(&self, repo_url: &str) -> Result<String, String> {
        // git ls-remote 같은 간단 체크
        let output = Command::new("git")
            .arg("ls-remote")
            .arg(repo_url)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!("Git access error: {}", repo_url));
        }

        // 그냥 “스캔 성공” 형태로 기본 컨텐츠 반환
        Ok(format!("SCANNED-REPO: {}", repo_url))
    }
}
