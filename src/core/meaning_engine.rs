pub struct MeaningEngine;

impl MeaningEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> MeaningGraph {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // 1) 토큰 → 노드 변환
        for (i, tok) in tokens.iter().enumerate() {
            nodes.push(MeaningNode {
                id: i,
                token: tok.to_string(),
            });

            // 2) 인접 토큰끼리 의미 edge 생성
            if i > 0 {
                edges.push(MeaningEdge {
                    from: i - 1,
                    to: i,
                    relation: "sequence".to_string(),
                });
            }
        }

        MeaningGraph { nodes, edges }
    }
}
