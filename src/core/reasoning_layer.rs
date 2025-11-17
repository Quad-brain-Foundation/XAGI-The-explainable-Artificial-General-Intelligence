use super::MeaningGraph;

pub struct ReasoningLayer;

impl ReasoningLayer {
    pub fn new() -> Self {
        Self
    }

    pub fn infer(&self, graph: &MeaningGraph) -> String {
        format!("Reasoning over {} nodes and {} edges",
            graph.nodes.len(),
            graph.edges.len()
        )
    }
}
