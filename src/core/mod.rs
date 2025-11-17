pub mod meaning_engine;
pub mod reasoning_layer;
pub mod memory;
pub mod self_recreator;
pub mod sp_backend;
pub mod sp_exec;
pub mod onnx_export;


#[derive(Debug, Clone)]
pub struct MeaningNode {
    pub id: usize,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct MeaningEdge {
    pub from: usize,
    pub to: usize,
    pub relation: String,
}

#[derive(Debug, Clone)]
pub struct MeaningGraph {
    pub nodes: Vec<MeaningNode>,
    pub edges: Vec<MeaningEdge>,
}
