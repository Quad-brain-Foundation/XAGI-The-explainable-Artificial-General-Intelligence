mod core;

use core::{
    meaning_engine::MeaningEngine,
    sp_backend::SpongeBackend,
    sp_exec::SpongeExecutor,
    reasoning_layer::ReasoningLayer,
    memory::MemoryStore,
    self_recreator::SelfRecreator,
    onnx_export::OnnxExporter,
};

fn main() {
    println!("🚀 XAGI: Explainable Artificial General Intelligence");

    // 1) 핵심 모듈 초기화
    let engine = MeaningEngine::new();
    let backend = SpongeBackend::new();
    let executor = SpongeExecutor::new();
    let reasoner = ReasoningLayer::new();
    let mut memory = MemoryStore::new();
    let recreator = SelfRecreator::new();
    let onnx = OnnxExporter::new();

    // 2) Self-recreation 부트스트랩
    recreator.bootstrap();

    // 3) 입력 텍스트
    let input = "XAGI explains itself using meaning graphs";

    println!("📥 Input: {}", input);

    // 4) 의미 그래프 생성
    let graph = engine.parse(input);

    // 5) SpongeLang AST 생성
    let sp_code = backend.generate(&graph);
    println!("\n🧽 Generated SpongeLang AST:\n{}\n", sp_code);

    // 6) Sponge-lang VM 실행 (있으면 실행, 없으면 에러 무시)
    match executor.run(&sp_code, "output.sp") {
        Ok(out) => {
            println!("🧠 SpongeVM Output:\n{}", out);
            memory.store(&out);
        }
        Err(err) => {
            println!("⚠️ SpongeVM not executed or error:\n{}\n(Continuing...)", err);
        }
    }

    // 7) ONNX 파일 export
    match onnx.export(&graph, "xagi_model.onnx") {
        Ok(_) => println!("📤 ONNX Exported → xagi_model.onnx"),
        Err(e) => println!("❌ ONNX Export Error: {}", e),
    }

    // 8) Reasoning 단계
    let reasoning_output = reasoner.infer(&graph);
    println!("\n🧩 Reasoning Output: {}", reasoning_output);
    memory.store(&reasoning_output);

    // 9) 메모리 출력
    println!("\n🧠 Memory State: {:?}", memory.recall());

    println!("\n✅ XAGI pipeline completed.");
}
