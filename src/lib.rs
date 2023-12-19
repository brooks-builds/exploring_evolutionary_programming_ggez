mod entity;

use entity::Entity;
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{self, DrawParam},
};

pub struct MainState {
    entity: Entity,
    width: f32,
    height: f32,
    friction: Vec2,
}

impl MainState {
    pub fn new(width: f32, height: f32) -> Self {
        let mut entity = Entity::new(25.0, 25.0);
        let force = Vec2::new(100.0, 0.0);
        let friction = Vec2::new(0.01, 0.01);

        entity.apply_force(force);

        Self {
            entity,
            width,
            height,
            friction,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut friction_force = self.entity.velocity * self.friction;

        friction_force *= -1.0;

        self.entity.apply_force(friction_force);

        self.entity.update();

        if self.entity.position.x + self.entity.size > self.width {
            self.entity.bounce_x();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Some(graphics::Color::BLACK));

        canvas.draw(
            &self.entity.render(&context)?,
            DrawParam::default().dest(self.entity.position),
        );

        canvas.finish(context)?;

        Ok(())
    }
}
