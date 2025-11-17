use std::process::Command;
use std::fs;

pub struct SpongeExecutor;

impl SpongeExecutor {
    pub fn new() -> Self {
        Self
    }

    /// AST → .sp 파일 저장 → C++ SpongeLang VM 실행
    pub fn run(&self, code: &str, out_path: &str) -> Result<String, String> {

        // 1) write to .sp file
        if let Err(e) = fs::write(out_path, code) {
            return Err(format!("File write error: {}", e));
        }

        // 2) call sponge-lang VM
        // VM 실행 파일 이름은 네 repo 기준으로 변경 가능
        let output = Command::new("./spongevm")
            .arg(out_path)
            .output()
            .map_err(|e| format!("VM execution error: {}", e))?;

        // 3) parse result
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
