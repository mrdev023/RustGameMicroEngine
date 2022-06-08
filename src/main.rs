use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

fn main() {
    if let Err(err) = TermLogger::init(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto) {
        println!("Failed to start logger : {}", err);
    }

    let engine = engine_core::Engine::new("Test 123");
    pollster::block_on(engine.run());
}
