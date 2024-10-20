use nalgebra as na;

pub struct David {
    pub position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

impl David {
    pub fn new() -> Self {
        David {
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Vector2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        
        // Simple boundary checking
        if self.position.x < -100.0 || self.position.x > 100.0 {
            self.velocity.x *= -0.8; // Bounce off walls with some dampening
        }
        if self.position.y < -100.0 || self.position.y > 100.0 {
            self.velocity.y *= -0.8; // Bounce off floor/ceiling with some dampening
        }

        // Apply friction
        self.velocity *= 0.98;
    }

    pub fn apply_force(&mut self, force: f32) {
        self.velocity += na::Vector2::new(force, -force * 0.5); // Apply force with some upward component
    }
}
