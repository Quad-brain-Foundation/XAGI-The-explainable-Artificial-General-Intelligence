mod core;

use core::{
    meaning_engine::MeaningEngine,
    reasoning_layer::ReasoningLayer,
    memory::MemoryStore,
    self_recreator::SelfRecreator,
};

fn main() {
    let engine = MeaningEngine::new();
    let reasoner = ReasoningLayer::new();
    let mut memory = MemoryStore::new();
    let recreator = SelfRecreator::new();

    recreator.bootstrap();

    let input = "XAGI explains itself";
    let graph = engine.parse(input);
    let out = reasoner.infer(&graph);

    memory.store(&out);

    println!("📌 Output: {}", out);
    println!("🧠 Memory: {:?}", memory.recall());
}
