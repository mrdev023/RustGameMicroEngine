mod engine;
pub use engine::Engine;

mod state;
pub use state::State;

pub mod render;

use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

fn main() {
    if let Err(err) = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto) {
        println!("Failed to start logger : {}", err);
    }

    let engine = Engine::new("Test 123");
    pollster::block_on(engine.run());
}
