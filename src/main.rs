use tuto1::run;

fn main() {
    async_std::task::block_on(run());
}
