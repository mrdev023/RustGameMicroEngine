mod render;
mod entities;
mod math;

use entities::Entity;
use entities::Player;
use render::debug;
use render::vulkan::test;

fn main() {
    // let mut player = Player::new();
    // debug(&player);
    // player.get_transform().translate(10.0, 20.0);
    // debug(&player);
    test();
}
