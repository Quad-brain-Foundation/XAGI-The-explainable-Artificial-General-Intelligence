use super::{MeaningGraph, MeaningNode, MeaningEdge};

pub struct SpongeBackend;

impl SpongeBackend {
    pub fn new() -> Self {
        Self
    }

    /// Convert MeaningGraph → SpongeLang AST (full)
    pub fn generate(&self, graph: &MeaningGraph) -> String {
        let mut out = String::new();

        out.push_str("(sp-ast\n");

        // Nodes
        out.push_str("  (nodes\n");
        for MeaningNode { id, token } in &graph.nodes {
            out.push_str(&format!("    (node {} \"{}\")\n", id, escape(token)));
        }
        out.push_str("  )\n\n");

        // Edges
        out.push_str("  (links\n");
        for MeaningEdge { from, to, relation } in &graph.edges {
            out.push_str(&format!(
                "    (link {} {} \"{}\")\n",
                from, to, escape(relation)
            ));
        }
        out.push_str("  )\n");

        out.push_str(")\n");
        out
    }
}

/// escape strings for SpongeLang
fn escape(s: &str) -> String {
    s.replace("\"", "\\\"")
}
