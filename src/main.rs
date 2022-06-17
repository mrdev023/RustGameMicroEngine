mod state;
pub use state::State;

pub mod input;
pub mod meshs;
pub mod render;

use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

fn main() {
    if let Err(err) = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ) {
        println!("Failed to start logger : {}", err);
    }

    let engine = render::Window::new("Test 123");
    pollster::block_on(engine.run());
}
