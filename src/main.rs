use rust_micro_game_engine::run;

fn main() {
    async_std::task::block_on(run());
}
