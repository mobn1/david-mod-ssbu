use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;

pub struct David {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    sprite: graphics::Image,
}

impl David {
    pub fn new() -> Self {
        David {
            position: na::Point2::new(400.0, 300.0),
            velocity: na::Vector2::new(0.0, 0.0),
            sprite: graphics::Image::new(Context, "/david_sprite.svg").unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;

        // Simple boundary checking
        if self.position.x < 0.0 || self.position.x > 800.0 {
            self.velocity.x *= -1.0;
        }
        if self.position.y < 0.0 || self.position.y > 600.0 {
            self.velocity.y *= -1.0;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let draw_params = graphics::DrawParam::default()
            .dest(self.position)
            .offset(na::Point2::new(0.5, 0.5));
        graphics::draw(ctx, &self.sprite, draw_params)
    }

    pub fn apply_force(&mut self, force: na::Vector2<f32>) {
        self.velocity += force;
    }
}
