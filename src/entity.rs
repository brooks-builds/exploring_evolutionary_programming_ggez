use ggez::{
    glam::Vec2,
    graphics::{Color, DrawMode, Mesh},
    Context, GameResult,
};

pub struct Entity {
    pub position: Vec2,
    pub size: f32,
    color: Color,
    pub velocity: Vec2,
    acceleration: Vec2,
    pub is_alive: bool,
    pub aim_rotation: f32,
    pub fired: bool,
}

impl Entity {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        let position = Vec2::new(x, y);
        let color = Color::WHITE;
        let velocity = Vec2::ZERO;
        let acceleration = Vec2::ZERO;
        let is_alive = true;
        let aim_rotation = 0.0;
        let fired = false;

        Self {
            position,
            size,
            color,
            velocity,
            acceleration,
            is_alive,
            aim_rotation,
            fired,
        }
    }

    pub fn render(&self, context: &Context) -> GameResult<Mesh> {
        Mesh::new_circle(
            context,
            DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.size,
            0.1,
            self.color,
        )
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    pub fn bounce_y(&mut self) {
        self.velocity.y *= -1.0;
    }
}
