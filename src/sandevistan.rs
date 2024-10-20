pub struct Sandevistan {
    active: bool,
    duration: f32,
    cooldown: f32,
    max_duration: f32,
    max_cooldown: f32,
}

impl Sandevistan {
    pub fn new() -> Self {
        Sandevistan {
            active: false,
            duration: 0.0,
            cooldown: 0.0,
            max_duration: 3.0,  // 3 seconds of active time
            max_cooldown: 10.0, // 10 seconds cooldown
        }
    }

    pub fn activate(&mut self) {
        if self.cooldown == 0.0 {
            self.active = true;
            self.duration = self.max_duration;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.duration -= dt;
            if self.duration <= 0.0 {
                self.active = false;
                self.duration = 0.0;
                self.cooldown = self.max_cooldown;
            }
        } else if self.cooldown > 0.0 {
            self.cooldown -= dt;
            if self.cooldown < 0.0 {
                self.cooldown = 0.0;
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
