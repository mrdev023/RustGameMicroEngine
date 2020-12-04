use crate::entities::Entity;
use engine_math::Transform;

#[derive(Debug)]
pub struct Player {
    transform: Transform,
}

impl Player {
    pub fn new() -> Player {
        Player {
            transform: Transform::new(),
        }
    }
}

impl Entity for Player {
    fn get_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
