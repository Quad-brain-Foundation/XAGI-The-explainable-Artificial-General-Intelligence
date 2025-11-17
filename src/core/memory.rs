#[derive(Default)]
pub struct MemoryStore {
    pub logs: Vec<String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn store(&mut self, data: &str) {
        self.logs.push(data.to_string());
    }

    pub fn recall(&self) -> Vec<String> {
        self.logs.clone()
    }
}
