mod entity;

use std::f32::consts::PI;

use ::ggez::graphics::Color;
use bot::{game_info::GameInfo, Bot};
use entity::Entity;
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{self, DrawParam, PxScale},
    winit::event::VirtualKeyCode,
};

const DESIRED_FPS: u32 = 120;
const FAST_FPS: u32 = 1024;

pub struct MainState {
    players: Vec<Entity>,
    width: f32,
    height: f32,
    bot: Bot,
    target: Entity,
    bullet_speed: f32,
    bullets: Vec<Entity>,
    rotate_amount: f32,
    end_generation_on_tick: usize,
    end_generation_after_ticks: usize,
    max_hit_count: u8,
    normal_speed: bool,
    ticks: usize,
}

impl MainState {
    pub fn new(width: f32, height: f32) -> Self {
        let bot_population_size = 100;
        let bot = Bot::new(bot_population_size as u8);
        let mut target = Entity::new(width - 50.0, 50.0, 25.0);
        let bullet_speed = 15.0;
        let players = bot
            .population
            .iter()
            .map(|_individual| Entity::new(50.0, height / 2.0 - 50.0, 25.0))
            .collect();
        let bullets = vec![];
        let rotate_amount = 0.01;
        let end_generation_on_tick = 50;
        let end_generation_after_ticks = 10;
        let max_hit_count = 0;
        let normal_speed = true;
        let ticks = 0;

        target.apply_force(Vec2::new(0.0, 0.5));

        Self {
            players,
            width,
            height,
            bot,
            target,
            bullet_speed,
            bullets,
            rotate_amount,
            end_generation_on_tick,
            end_generation_after_ticks,
            max_hit_count,
            normal_speed,
            ticks,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let fps = if self.normal_speed {
            DESIRED_FPS
        } else {
            FAST_FPS
        };

        while context.time.check_update_time(fps) {
            let arena_size = Vec2::new(self.width, self.height);
            let target_position = self.target.position;
            let target_velocity = self.target.velocity;
            let target_size = self.target.size;
            let bullet_speed = self.bullet_speed;
            let mut bullet_fired = false;
            let mut target_hits = 0;

            for (player, individual) in self.players.iter_mut().zip(self.bot.population.iter()) {
                let game_info = GameInfo::new(
                    player.position,
                    arena_size,
                    target_position,
                    target_velocity,
                    target_size,
                    None,
                    bullet_speed,
                    player.aim_rotation,
                );
                let commands = individual.play(&game_info);

                for command in commands {
                    match command {
                        bot::command::Command::RotateLeft => {
                            player.aim_rotation -= self.rotate_amount;
                            player.aim_rotation = player.aim_rotation.clamp(-PI, PI);
                        }
                        bot::command::Command::RotateRight => {
                            player.aim_rotation += self.rotate_amount;
                            player.aim_rotation = player.aim_rotation.clamp(-PI, PI);
                        }
                        bot::command::Command::Fire => {
                            if player.cool_down > 0 {
                                continue;
                            }

                            let mut bullet = Entity::new(player.position.x, player.position.y, 5.0);
                            let mut aim = Vec2::from_angle(player.aim_rotation);

                            aim *= bullet_speed;
                            bullet.apply_force(aim);
                            self.bullets.push(bullet);

                            player.set_cool_down(2);
                            self.end_generation_on_tick += self.end_generation_after_ticks;
                        }
                        bot::command::Command::Nothing => (),
                    }
                }
            }

            for ((bullet, individual), player) in self
                .bullets
                .iter_mut()
                .zip(self.bot.population.iter_mut())
                .zip(self.players.iter())
            {
                let game_info = GameInfo::new(
                    player.position,
                    arena_size,
                    target_position,
                    target_velocity,
                    target_size,
                    Some(bullet.position),
                    bullet_speed,
                    player.aim_rotation,
                );

                individual.update(game_info);
                bullet.update();

                let bullet_distance_to_target = bullet.position.distance(self.target.position);
                if bullet_distance_to_target < target_size {
                    target_hits += 1;
                }
            }

            self.max_hit_count = self.max_hit_count.max(target_hits);

            if self.ticks == self.end_generation_on_tick {
                self.bullets.clear();
                self.bot.run();
                self.players.clear();
                self.players = self
                    .bot
                    .population
                    .iter()
                    .map(|_individual| Entity::new(50.0, arena_size.y / 2.0 - 50.0, 25.0))
                    .collect();

                self.end_generation_on_tick = self.ticks + self.end_generation_after_ticks;
            }
            // let target = &self.target;

            // if self.running {
            //     self.bullets
            //         .iter_mut()
            //         .zip(&mut self.players)
            //         .zip(&mut self.bot.population)
            //         .for_each(move |((bullet, player), individual)| {
            //             if bullet.is_alive {
            //                 bullet.update();
            //             }

            //             let bullet_distance_to_target =
            //                 (bullet.position - target.position).length().abs();
            //             if bullet_distance_to_target <= target_size
            //                 || bullet.is_out_of_arena(arena_size.x, arena_size.y)
            //             {
            //                 bullet.is_alive = false;
            //             }

            //             let game_info = GameInfo::new(
            //                 player.position,
            //                 arena_size,
            //                 target_position,
            //                 target_velocity,
            //                 target_size,
            //                 bullet.position,
            //                 bullet_speed,
            //                 player.aim_rotation,
            //             );
            //             individual.update(game_info);
            //         });

            //     let alive_bullets = self.bullets.iter().filter(|bullet| bullet.is_alive).count();
            //     if alive_bullets == 0 {
            //         self.running = false;
            //     }
            // } else {
            //     self.bot.run();
            //     self.bullets = self
            //         .players
            //         .iter()
            //         .zip(&self.bot.population)
            //         .map(|(player, individual)| {
            //             let mut bullet = Entity::new(player.position.x, player.position.y, 5.0);
            //             let game_info = GameInfo::new(
            //                 player.position,
            //                 arena_size,
            //                 target_position,
            //                 target_velocity,
            //                 target_size,
            //                 bullet.position,
            //                 bullet_speed,
            //                 player.aim_rotation,
            //             );
            //             let commands = individual.play(&game_info);

            //             bullet.apply_force(aim);

            //             bullet
            //         })
            //         .collect();
            //     self.running = true;
            // }

            self.target.update();
            if self.target.position.y + self.target.size > self.height
                || self.target.position.y - self.target.size < 0.0
            {
                self.target.bounce_y();
            }

            self.ticks += 1;
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(context, Some(graphics::Color::BLACK));

        for entity in self.players.iter() {
            canvas.draw(
                &entity.render(&context)?,
                DrawParam::default().dest(entity.position),
            );
            let aim = Vec2::from_angle(entity.aim_rotation);
            let points = [entity.position, entity.position + (aim * 50.0)];
            let line = graphics::Mesh::new_line(context, &points, 2.0, Color::WHITE)?;
            canvas.draw(&line, DrawParam::default());
        }

        for bullet in self.bullets.iter() {
            let mesh = bullet.render(context)?;
            let params = DrawParam::default().dest(bullet.position);
            canvas.draw(&mesh, params);
        }

        let target_mesh = self.target.render(context)?;
        let target_params = DrawParam::default().dest(self.target.position.clone());
        canvas.draw(&target_mesh, target_params);

        let mut text = graphics::Text::new(format!(
            "generations: {}",
            self.bot.generation_count.to_string()
        ));
        text.set_scale(PxScale::from(64.0));
        canvas.draw(&text, DrawParam::default().dest(Vec2::new(10.0, 500.0)));

        let mut bullet_count = graphics::Text::new(format!(
            "bullet fired this generation: {}",
            self.bullets.len().to_string()
        ));
        bullet_count.set_scale(PxScale::from(64.0));
        canvas.draw(
            &bullet_count,
            DrawParam::default().dest(Vec2::new(10.0, 400.0)),
        );

        let mut max_hit_count = graphics::Text::new(format!(
            "max hits in a generation: {}",
            self.max_hit_count.to_string()
        ));
        max_hit_count.set_scale(PxScale::from(64.0));
        canvas.draw(
            &max_hit_count,
            DrawParam::default().dest(Vec2::new(10.0, 10.0)),
        );

        canvas.finish(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::Space => self.normal_speed = !self.normal_speed,
                _ => (),
            }
        }

        Ok(())
    }
}
