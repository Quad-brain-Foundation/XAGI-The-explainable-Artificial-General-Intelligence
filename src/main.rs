mod core;

use core::{
    meaning_engine::MeaningEngine,
    reasoning_layer::ReasoningLayer,
    memory::MemoryStore,
    self_recreator::SelfRecreator,
    sp_backend::SpongeBackend,
};

fn main() {
    let engine = MeaningEngine::new();
    let backend = SpongeBackend::new();
    let reasoner = ReasoningLayer::new();
    let mut memory = MemoryStore::new();
    let recreator = SelfRecreator::new();

    recreator.bootstrap();

    let input = "XAGI explains itself";
    let graph = engine.parse(input);

    let sp_code = backend.generate(&graph);
    println!("🧽 SpongeLang Code:\n{}", sp_code);

    let result = reasoner.infer(&graph);
    memory.store(&result);

    println!("📌 Reason: {}", result);
}
