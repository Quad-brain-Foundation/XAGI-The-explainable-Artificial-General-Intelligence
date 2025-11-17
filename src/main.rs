mod core;
mod runtime;
mod utils;

use core::{meaning_engine::MeaningEngine, reasoning_layer::ReasoningLayer};
use runtime::{exec_context::ExecContext, meta_recreation::SelfRecreator};

fn main() {
    println!("🧠 XAGI: Explainable Artificial General Intelligence - Booting...");

    let mut context = ExecContext::new();
    let mut engine = MeaningEngine::new();
    let mut reasoner = ReasoningLayer::new();

    // initialize self model
    let mut self_recreator = SelfRecreator::new();
    self_recreator.bootstrap();

    // run example reasoning cycle
    let input = "Explain the reason behind self-recreation";
    let meaning = engine.parse(input);
    let result = reasoner.infer(meaning, &mut context);

    println!("🧩 Result:\n{}", result);
}
