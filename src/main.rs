mod core;

use core::{
    meaning_engine::MeaningEngine,
    sp_backend::SpongeBackend,
    sp_exec::SpongeExecutor,
};

fn main() {
    let engine = MeaningEngine::new();
    let backend = SpongeBackend::new();
    let exec = SpongeExecutor::new();

    // 입력
    let input = "XAGI explains itself using meaning";
    
    // 1) 의미 분석
    let graph = engine.parse(input);

    // 2) SpongeLang AST 생성
    let sp_code = backend.generate(&graph);
    println!("▶ Generated SpongeLang AST:\n{}\n", sp_code);

    // 3) Sponge-lang VM 실행
    match exec.run(&sp_code, "output.sp") {
        Ok(output) => {
            println!("🧽 SpongeVM Output:\n{}", output);
        }
        Err(err) => {
            println!("❌ SpongeVM Error:\n{}", err);
        }
    }
}
