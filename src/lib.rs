mod entity;

use bot::Bot;
use entity::Entity;
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{self, DrawParam, PxScale},
};

pub struct MainState {
    entities: Vec<Entity>,
    width: f32,
    friction: Vec2,
    running: bool,
    bot: Bot,
}

impl MainState {
    pub fn new(width: f32, _height: f32) -> Self {
        let friction = Vec2::new(0.01, 0.01);
        let entities = vec![];
        let running = false;
        let bot = Bot::new();

        Self {
            entities,
            width,
            friction,
            running,
            bot,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if self.running {
            let mut still_moving = 0;

            for entity in self.entities.iter_mut() {
                let mut friction_force = entity.velocity * self.friction;
                friction_force *= -1.0;
                entity.apply_force(friction_force);
                entity.update();

                if entity.position.x + entity.size > self.width {
                    entity.bounce_x();
                }

                if entity.position.x - entity.size > 0.0 && entity.still_moving() {
                    still_moving += 1;
                }
            }

            if still_moving == 0 {
                self.running = false;
                let xs = self
                    .entities
                    .iter()
                    .map(|entity| entity.position.x)
                    .collect();
                self.bot.run(xs)
            }
        } else {
            let force_xs = self.bot.play();
            self.entities = force_xs
                .into_iter()
                .enumerate()
                .map(|(index, force_x)| {
                    let y = (index as f32 + 1.0) * 50.0;
                    let mut entity = Entity::new(25.0, y);
                    let force = Vec2::new(force_x, 0.0);

                    entity.apply_force(force);

                    entity
                })
                .collect();
            self.running = true;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Some(graphics::Color::BLACK));

        for entity in self.entities.iter() {
            canvas.draw(
                &entity.render(&context)?,
                DrawParam::default().dest(entity.position),
            );
        }

        let mut text = graphics::Text::new(self.bot.generation_count.to_string());
        text.set_scale(PxScale::from(64.0));

        canvas.draw(&text, DrawParam::default().dest(Vec2::new(10.0, 500.0)));

        canvas.finish(context)?;

        Ok(())
    }
}
