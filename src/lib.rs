mod entity;

use std::{f32::consts::PI, io::Write, path::Path};

use ::ggez::graphics::Color;
use bot::{game_info::GameInfo, Bot};
use entity::Entity;
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{self, DrawParam, PxScale},
    winit::event::VirtualKeyCode,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

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
    rng: ThreadRng,
}

impl MainState {
    pub fn new(width: f32, height: f32) -> Self {
        let bot_population_size = 50;
        let bot = Bot::new(bot_population_size as u8);
        let mut target = Entity::new(width - 50.0, 50.0, 25.0);
        let bullet_speed = 15.0;
        let players = bot
            .population
            .iter()
            .map(|_individual| Entity::new(width / 2.0 - 50.0, height / 2.0 - 50.0, 25.0))
            .collect();
        let bullets = vec![];
        let rotate_amount = 0.01;
        let end_generation_on_tick = 500;
        let end_generation_after_ticks = 500;
        let max_hit_count = 0;
        let normal_speed = true;
        let ticks = 0;
        let rng = thread_rng();

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
            rng,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let arena_size = Vec2::new(self.width, self.height);
        let target_position = self.target.position;
        let target_velocity = self.target.velocity;
        let target_size = self.target.size;
        let bullet_speed = self.bullet_speed;
        let mut target_hits = 0;
        let ticks = context.time.ticks();

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
                        if player.fired {
                            continue;
                        }

                        let mut bullet = Entity::new(player.position.x, player.position.y, 5.0);
                        let mut aim = Vec2::from_angle(player.aim_rotation);

                        aim *= bullet_speed;
                        bullet.apply_force(aim);
                        self.bullets.push(bullet);

                        player.fired = true;
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

            if bullet.position.x + bullet.size < 0.0
                || bullet.position.y + bullet.size < 0.0
                || bullet.position.x - bullet.size > arena_size.x
                || bullet.position.y - bullet.size > arena_size.y
            {
                bullet.is_alive = false;
            }

            let bullet_distance_to_target = bullet.position.distance(self.target.position);
            if bullet_distance_to_target < target_size {
                target_hits += 1;
            }
        }

        self.max_hit_count = self.max_hit_count.max(target_hits);

        let alive_bullets = self.bullets.iter().filter(|bullet| bullet.is_alive).count();
        let done_early =
            self.bot.population_size as usize == self.bullets.len() && alive_bullets == 0;

        if done_early || ticks % self.end_generation_after_ticks == 0 {
            self.bullets.clear();
            self.bot.run();
            self.players.clear();
            self.players = self
                .bot
                .population
                .iter()
                .map(|_individual| {
                    Entity::new(arena_size.x / 2.0 - 50.0, arena_size.y / 2.0 - 50.0, 25.0)
                })
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

        let random_force = Vec2::new(
            self.rng.gen_range(-0.1..=0.1),
            self.rng.gen_range(-0.1..=0.1),
        );
        self.target.apply_force(random_force);
        self.target.update();
        if self.target.position.y + self.target.size > self.height
            || self.target.position.y - self.target.size < 0.0
        {
            self.target.bounce_y();
        }
        if self.target.position.x + self.target.size > self.width
            || self.target.position.x - self.target.size < 0.0
        {
            self.target.bounce_x();
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
                VirtualKeyCode::Space => {
                    let weights = format!("{:?}", self.bot.population);
                    write_weights_to_disk(weights);
                }
                _ => (),
            }
        }

        Ok(())
    }
}

fn write_weights_to_disk(weights: String) {
    let path = Path::new("weights.txt");
    let Ok(mut weights_file) = std::fs::File::options()
        .create_new(true)
        .write(true)
        .open(path)
    else {
        eprintln!("Error opening 'weights.txt' file");
        return;
    };

    if let Err(error) = weights_file.write_all(weights.as_bytes()) {
        eprintln!("Error writing weights to file: {error}");
    } else {
        println!("Weights written to 'weights.txt'");
    }
}
