#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 {
            x: 0.0,
            y: 0.0
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, b: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + b.x,
            y: self.y + b.y
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, b: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - b.x,
            y: self.y - b.y
        }
    }
}