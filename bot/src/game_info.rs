use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct GameInfo {
    pub position: Vec2,
    pub arena_size: Vec2,
    pub target_position: Vec2,
    pub target_velocity: Vec2,
    pub target_size: f32,
    pub bullet_position: Vec2,
    pub bullet_speed: f32,
}

impl GameInfo {
    pub fn new(
        position: Vec2,
        arena_size: Vec2,
        target_position: Vec2,
        target_velocity: Vec2,
        target_size: f32,
        bullet_position: Vec2,
        bullet_speed: f32,
    ) -> Self {
        Self {
            position,
            arena_size,
            target_position,
            target_velocity,
            target_size,
            bullet_position,
            bullet_speed,
        }
    }
}
