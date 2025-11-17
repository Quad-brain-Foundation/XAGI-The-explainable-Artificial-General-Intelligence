mod core;

use core::{
    meaning_engine::MeaningEngine,
    reasoning_layer::ReasoningLayer,
    memory::MemoryStore,
    self_recreator::SelfRecreator,
    sp_backend::SpongeBackend,
    onnx_export::OnnxExporter,
    auto_scan::RepoAutoScanner,
};

fn main() {
    println!("🚀 XAGI: Global Meaning Learner");

    // 코어 엔진들
    let engine = MeaningEngine::new();
    let backend = SpongeBackend::new();
    let reasoner = ReasoningLayer::new();
    let mut memory = MemoryStore::new();
    let recreator = SelfRecreator::new();
    let onnx = OnnxExporter::new();

    recreator.bootstrap();

    // ======================================================
    // 1) Freeing-the-Lang 전체 리포 자동 스캔
    // ======================================================
    let scanner = RepoAutoScanner::new();

    let repos = vec![
        "https://github.com/Freeing-the-Lang/Go-like-rust",
        "https://github.com/Freeing-the-Lang/Sponge-lang",
        "https://github.com/Freeing-the-Lang/Swift-with-no-llvm",
        "https://github.com/Freeing-the-Lang/Rust-like-cplusplus",
        "https://github.com/Freeing-the-Lang/Pure-rust-no-llvm",
        "https://github.com/Quad-brain-Foundation/XAGI-The-explainable-Artificial-General-Intelligence"
    ];

    println!("📡 Auto-Scanning {} repos...", repos.len());

    let mut all_sources = String::new();
    for repo in repos {
        if let Ok(code) = scanner.fetch(repo) {
            println!("📥 Ingested from: {}", repo);
            all_sources.push_str(&code);
        } else {
            println!("⚠️ Scan failed: {}", repo);
        }
    }

    // ======================================================
    // 2) 자동 의미 학습
    // ======================================================
    println!("🧠 Meaning learning from all ingested sources...");
    let graph = engine.parse(&all_sources);

    // ======================================================
    // 3) XAGI Reasoning
    // ======================================================
    let reasoning_output = reasoner.infer(&graph);
    memory.store(&reasoning_output);

    // ======================================================
    // 4) SpongeLang AST 출력
    // ======================================================
    let sp_code = backend.generate(&graph);
    println!("🧽 SpongeLang AST Ready.");

    // ======================================================
    // 5) ONNX Export
    // ======================================================
    if let Ok(_) = onnx.export(&graph, "xagi_model.onnx") {
        println!("📤 Exported ONNX model → xagi_model.onnx");
    }

    // ======================================================
    // 6) Memory 확인
    // ======================================================
    println!("🧠 Memory Snapshot: {:?}", memory.recall());

    println!("✨ XAGI auto-scan pipeline completed.");
}
