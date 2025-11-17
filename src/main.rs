mod core;

use core::{
    meaning_engine::MeaningEngine,
    sp_backend::SpongeBackend,
    sp_exec::SpongeExecutor,
    reasoning_layer::ReasoningLayer,
    memory::MemoryStore,
    self_recreator::SelfRecreator,
    onnx_export::OnnxExporter,
    auto_scan::RepoAutoScanner,
};

fn main() {
    println!("🚀 XAGI Auto-Scan Mode");

    let engine = MeaningEngine::new();
    let backend = SpongeBackend::new();
    let executor = SpongeExecutor::new();
    let reasoner = ReasoningLayer::new();
    let mut memory = MemoryStore::new();
    let recreator = SelfRecreator::new();
    let onnx = OnnxExporter::new();
    let scanner = RepoAutoScanner::new();

    recreator.bootstrap();

    // 🔥 Freeing-the-Lang 전체 자동 스캔 (예시)
    let repos = [
        "https://github.com/Freeing-the-Lang/Go-like-rust",
        "https://github.com/Freeing-the-Lang/Sponge-lang",
        "https://github.com/Freeing-the-Lang/Swift-with-no-llvm",
    ];

    for repo in repos {
        println!("🔍 Scanning {}", repo);

        if let Ok(code) = scanner.fetch(repo) {
            println!("📥 Repo Data: {}", code);

            let graph = engine.parse(&code);
            let ast = backend.generate(&graph);
            println!("🧽 AST:\n{}", ast);

            memory.store(&ast);

            let _ = onnx.export(&graph, "xagi_autoscan.onnx");
        }
    }

    println!("🧠 Memory: {:?}", memory.recall());

    println!("✅ Auto Scan Completed.");
}
