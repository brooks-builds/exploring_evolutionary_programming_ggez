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
}

impl Entity {
    pub fn new(x: f32, y: f32) -> Self {
        let position = Vec2::new(x, y);
        let size = 25.0;
        let color = Color::WHITE;
        let velocity = Vec2::ZERO;
        let acceleration = Vec2::ZERO;

        Self {
            position,
            size,
            color,
            velocity,
            acceleration,
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

    pub fn bounce_x(&mut self) {
        self.velocity.x *= -1.0;
    }

    pub fn still_moving(&self) -> bool {
        self.velocity.x.abs() > 0.01
    }
}
