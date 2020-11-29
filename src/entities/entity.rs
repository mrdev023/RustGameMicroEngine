use crate::math::Transform;

pub trait Entity {
    fn get_transform(&mut self) -> &mut Transform;
}