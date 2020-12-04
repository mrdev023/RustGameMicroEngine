use super::Vec2;

#[derive(Debug)]
pub struct Transform {
    position: Vec2,
    rotation: Vec2,
    scale: Vec2
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec2::new(),
            rotation: Vec2::new(),
            scale: Vec2::new()
        }
    }

    pub fn get_rotation(&self) -> &Vec2 {
        &self.rotation
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_scale(&self) -> &Vec2 {
        &self.scale
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.position.x += x;
        self.position.y += y;
    }

    pub fn rotate(&mut self, x: f64, y: f64) {
        self.rotation.x += x;
        self.rotation.y += y;
    }
}
