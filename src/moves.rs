use nalgebra as na;

#[derive(Debug)]
pub enum Move {
    Jab,
    DashAttack,
    TiltUp,
    TiltSide,
    TiltDown,
    AerialNeutral,
    AerialForward,
    AerialBack,
    AerialUp,
    AerialDown,
    SmashSide,
    SmashUp,
    SmashDown,
    SpecialNeutral,
    SpecialSide,
    SpecialUp,
    SpecialDown,
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
            knockback: na::Vector2::new(1.0, 0.5),
            startup_frames: 2,
            active_frames: 2,
            recovery_frames: 6,
        },
        Move::DashAttack => MoveData {
            damage: 8.0,
            knockback: na::Vector2::new(2.0, 1.0),
            startup_frames: 5,
            active_frames: 4,
            recovery_frames: 15,
        },
        Move::TiltUp => MoveData {
            damage: 6.0,
            knockback: na::Vector2::new(0.5, 2.0),
            startup_frames: 4,
            active_frames: 3,
            recovery_frames: 10,
        },
        Move::TiltSide => MoveData {
            damage: 7.0,
            knockback: na::Vector2::new(1.5, 0.5),
            startup_frames: 5,
            active_frames: 3,
            recovery_frames: 12,
        },
        Move::TiltDown => MoveData {
            damage: 5.0,
            knockback: na::Vector2::new(1.0, -0.5),
            startup_frames: 3,
            active_frames: 4,
            recovery_frames: 8,
        },
        Move::AerialNeutral => MoveData {
            damage: 6.0,
            knockback: na::Vector2::new(1.0, 1.0),
            startup_frames: 4,
            active_frames: 4,
            recovery_frames: 10,
        },
        Move::AerialForward => MoveData {
            damage: 9.0,
            knockback: na::Vector2::new(2.0, 0.5),
            startup_frames: 6,
            active_frames: 3,
            recovery_frames: 15,
        },
        Move::AerialBack => MoveData {
            damage: 10.0,
            knockback: na::Vector2::new(-2.0, 1.0),
            startup_frames: 7,
            active_frames: 3,
            recovery_frames: 18,
        },
        Move::AerialUp => MoveData {
            damage: 7.0,
            knockback: na::Vector2::new(0.5, 2.5),
            startup_frames: 5,
            active_frames: 3,
            recovery_frames: 12,
        },
        Move::AerialDown => MoveData {
            damage: 11.0,
            knockback: na::Vector2::new(0.5, -2.5),
            startup_frames: 8,
            active_frames: 4,
            recovery_frames: 20,
        },
        Move::SmashSide => MoveData {
            damage: 15.0,
            knockback: na::Vector2::new(3.0, 1.0),
            startup_frames: 12,
            active_frames: 3,
            recovery_frames: 25,
        },
        Move::SmashUp => MoveData {
            damage: 14.0,
            knockback: na::Vector2::new(0.5, 3.5),
            startup_frames: 10,
            active_frames: 3,
            recovery_frames: 22,
        },
        Move::SmashDown => MoveData {
            damage: 16.0,
            knockback: na::Vector2::new(1.0, -3.0),
            startup_frames: 14,
            active_frames: 4,
            recovery_frames: 28,
        },
        Move::SpecialNeutral => MoveData {
            damage: 0.0, // Sandevistan activation
            knockback: na::Vector2::new(0.0, 0.0),
            startup_frames: 1,
            active_frames: 180, // 3 seconds at 60 FPS
            recovery_frames: 60,
        },
        Move::SpecialSide => MoveData {
            damage: 10.0,
            knockback: na::Vector2::new(2.5, 0.5),
            startup_frames: 8,
            active_frames: 6,
            recovery_frames: 18,
        },
        Move::SpecialUp => MoveData {
            damage: 12.0,
            knockback: na::Vector2::new(0.5, 3.0),
            startup_frames: 6,
            active_frames: 8,
            recovery_frames: 20,
        },
        Move::SpecialDown => MoveData {
            damage: -5.0, // Self-damage for Chrome Overload
            knockback: na::Vector2::new(0.0, 0.0),
            startup_frames: 10,
            active_frames: 30,
            recovery_frames: 40,
        },
    }
}
