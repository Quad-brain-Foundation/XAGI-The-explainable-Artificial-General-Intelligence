use super::{MeaningGraph, MeaningNode};

pub struct SpongeBackend;

impl SpongeBackend {
    pub fn new() -> Self {
        Self
    }

    /// meaning graph → 기본 SpongeLang 코드로 변환
    pub fn generate(&self, graph: &MeaningGraph) -> String {
        let mut out = String::new();

        out.push_str("(sp-code\n");

        for MeaningNode { id, token } in &graph.nodes {
            out.push_str(&format!("  (node {} \"{}\")\n", id, token));
        }

        out.push_str(")");
        out
    }
}
