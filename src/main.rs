use ggez::{Context, GameResult, graphics, timer};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{self, KeyCode};

mod character;
mod moves;
mod sandevistan;

use character::David;
use sandevistan::Sandevistan;

struct GameState {
    david: David,
    sandevistan: Sandevistan,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            david: David::new(),
            sandevistan: Sandevistan::new(),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.sandevistan.activate();
            }

            let time_factor = if self.sandevistan.is_active() { 0.2 } else { 1.0 };
            self.david.update(dt * time_factor);
            self.sandevistan.update(dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.2, 0.3, 1.0));

        self.david.draw(ctx)?;

        if self.sandevistan.is_active() {
            // Draw Sandevistan effect
            let rect = graphics::Rect::new(0.0, 0.0, 800.0, 600.0);
            let sandevistan_overlay = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                graphics::Color::new(0.9, 0.3, 0.3, 0.2),
            )?;
            graphics::draw(ctx, &sandevistan_overlay, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("david_mod_prototype", "Cyberpunk Edgerunners");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
