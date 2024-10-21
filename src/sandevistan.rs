use std::time::Duration;

pub struct Sandevistan {
    active: bool,
    duration: Duration,
    cooldown: Duration,
    max_duration: Duration,
    max_cooldown: Duration,
    gauge: f32,
}

impl Sandevistan {
    pub fn new() -> Self {
        Sandevistan {
            active: false,
            duration: Duration::from_secs(0),
            cooldown: Duration::from_secs(0),
            max_duration: Duration::from_secs(3),
            max_cooldown: Duration::from_secs(10),
            gauge: 100.0,
        }
    }

    #[allow(dead_code)]
    pub fn activate(&mut self) -> bool {
        if self.cooldown == Duration::from_secs(0) && self.gauge >= 30.0 {
            self.active = true;
            self.duration = self.max_duration;
            self.gauge -= 30.0;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, dt: Duration) {
        if self.active {
            self.duration = self.duration.saturating_sub(dt);
            if self.duration == Duration::from_secs(0) {
                self.active = false;
                self.cooldown = self.max_cooldown;
            }
        } else if self.cooldown > Duration::from_secs(0) {
            self.cooldown = self.cooldown.saturating_sub(dt);
        }

        // Recharge the Sandevistan Gauge
        self.gauge = (self.gauge + 0.5).min(100.0);
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    #[allow(dead_code)]
    pub fn get_gauge(&self) -> f32 {
        self.gauge
    }
}
