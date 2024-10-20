use ggez::nalgebra as na;

pub enum Move {
    Jab,
    DashAttack,
    SideSmash,
    NeutralSpecial,
    SideSpecial,
    UpSpecial,
    DownSpecial,
}

pub struct MoveData {
    pub damage: f32,
    pub knockback: na::Vector2<f32>,
    pub startup_frames: u32,
    pub active_frames: u32,
    pub recovery_frames: u32,
}

pub fn get_move_data(move_type: Move) -> MoveData {
    match move_type {
        Move::Jab => MoveData {
            damage: 3.0,
            knockback: na::Vector2::new(0.5, 0.2),
            startup_frames: 2,
            active_frames: 2,
            recovery_frames: 6,
        },
        Move::DashAttack => MoveData {
            damage: 8.0,
            knockback: na::Vector2::new(1.0, 0.5),
            startup_frames: 5,
            active_frames: 4,
            recovery_frames: 15,
        },
        Move::SideSmash => MoveData {
            damage: 15.0,
            knockback: na::Vector2::new(2.0, 1.0),
            startup_frames: 12,
            active_frames: 3,
            recovery_frames: 25,
        },
        Move::NeutralSpecial => MoveData {
            damage: 0.0, // Sandevistan activation
            knockback: na::Vector2::new(0.0, 0.0),
            startup_frames: 1,
            active_frames: 180, // 3 seconds at 60 FPS
            recovery_frames: 60,
        },
        Move::SideSpecial => MoveData {
            damage: 10.0,
            knockback: na::Vector2::new(1.5, 0.5),
            startup_frames: 8,
            active_frames: 6,
            recovery_frames: 18,
        },
        Move::UpSpecial => MoveData {
            damage: 12.0,
            knockback: na::Vector2::new(0.5, 2.0),
            startup_frames: 6,
            active_frames: 8,
            recovery_frames: 20,
        },
        Move::DownSpecial => MoveData {
            damage: -5.0, // Self-damage for Chrome Overload
            knockback: na::Vector2::new(0.0, 0.0),
            startup_frames: 10,
            active_frames: 30,
            recovery_frames: 40,
        },
    }
}
