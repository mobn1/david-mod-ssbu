// Super Smash Bros Ultimate Mod for David from Cyberpunk Edgerunners

use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::*;

// Importing our existing modules
mod character;
mod moves;
mod sandevistan;

use character::David;
use moves::{Move, get_move_data};
use sandevistan::Sandevistan;

// Global variables for David's state
static mut DAVID: Option<David> = None;
static mut SANDEVISTAN: Option<Sandevistan> = None;
static mut CHROME_BALANCE: f32 = 50.0;

pub fn init_david() {
    unsafe {
        DAVID = Some(David::new());
        SANDEVISTAN = Some(Sandevistan::new());
        CHROME_BALANCE = 50.0;
        println!("David Mod: David's initial state set");
    }
}

#[skyline::hook(offset = 0x37BAD0)]
pub unsafe fn david_init(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        println!("David Mod: Initializing David for fighter");
        if let Some(david) = &mut DAVID {
            // Set initial position, etc.
            david.position = fighter.global_table[0x1].get_vec2();
        }
    }
    call_original!(fighter)
}

#[skyline::hook(offset = 0x37BAE0)]
pub unsafe fn david_frame(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        if let Some(david) = &mut DAVID {
            david.update();
            println!("David Mod: Updated David's position to {:?}", david.position);
        }
        if let Some(sandevistan) = &mut SANDEVISTAN {
            sandevistan.update(std::time::Duration::from_millis(16));
            println!("David Mod: Sandevistan status: {}", if sandevistan.is_active() { "Active" } else { "Inactive" });
        }
        CHROME_BALANCE = update_chrome_balance(CHROME_BALANCE);
        println!("David Mod: Chrome Balance updated to {:.2}", CHROME_BALANCE);
    }
    call_original!(fighter)
}

#[skyline::hook(offset = 0x37BBF0)]
pub unsafe fn david_move(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        println!("David Mod: Processing move with status kind: {}", status_kind);
        match status_kind {
            // Basic attacks
            *FIGHTER_STATUS_KIND_ATTACK => execute_move(fighter, &Move::Jab),
            *FIGHTER_STATUS_KIND_ATTACK_DASH => execute_move(fighter, &Move::DashAttack),
            
            // Tilt attacks
            *FIGHTER_STATUS_KIND_ATTACK_S3 => execute_move(fighter, &Move::TiltSide),
            *FIGHTER_STATUS_KIND_ATTACK_HI3 => execute_move(fighter, &Move::TiltUp),
            *FIGHTER_STATUS_KIND_ATTACK_LW3 => execute_move(fighter, &Move::TiltDown),
            
            // Aerial attacks
            *FIGHTER_STATUS_KIND_ATTACK_AIR => {
                if ControlModule::get_stick_y(fighter.module_accessor) > 0.5 {
                    execute_move(fighter, &Move::AerialUp)
                } else if ControlModule::get_stick_y(fighter.module_accessor) < -0.5 {
                    execute_move(fighter, &Move::AerialDown)
                } else if ControlModule::get_stick_x(fighter.module_accessor).abs() > 0.5 {
                    if fighter.global_table[0x15].get_i32() & 0x1 == 0 {
                        execute_move(fighter, &Move::AerialForward)
                    } else {
                        execute_move(fighter, &Move::AerialBack)
                    }
                } else {
                    execute_move(fighter, &Move::AerialNeutral)
                }
            },
            
            // Smash attacks
            *FIGHTER_STATUS_KIND_ATTACK_S4 => execute_move(fighter, &Move::SmashSide),
            *FIGHTER_STATUS_KIND_ATTACK_HI4 => execute_move(fighter, &Move::SmashUp),
            *FIGHTER_STATUS_KIND_ATTACK_LW4 => execute_move(fighter, &Move::SmashDown),
            
            // Special moves
            *FIGHTER_STATUS_KIND_SPECIAL_N => activate_sandevistan(fighter),
            *FIGHTER_STATUS_KIND_SPECIAL_S => execute_move(fighter, &Move::SpecialSide),
            *FIGHTER_STATUS_KIND_SPECIAL_HI => execute_move(fighter, &Move::SpecialUp),
            *FIGHTER_STATUS_KIND_SPECIAL_LW => use_chrome_overload(fighter),
            
            _ => {}
        }
    }
    call_original!(fighter)
}

fn execute_move(fighter: &mut L2CFighterCommon, move_type: &Move) {
    println!("David Mod: Executing move: {:?}", move_type);
    let move_data = get_move_data(move_type);
    if let Some(david) = &mut DAVID {
        david.apply_force(move_data.knockback.x);
    }
    
    // Apply damage and knockback using SSBU's built-in functions
    DamageModule::add_damage(fighter.module_accessor, move_data.damage, 0);
    let angle = move_data.knockback.y.atan2(move_data.knockback.x);
    let speed = move_data.knockback.magnitude();
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, speed, angle, 0, 0, 0);
    sv_kinetic_energy!(set_speed, fighter);
    
    // Set move properties
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(&format!("move_{:?}", move_type)),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    
    // Set frame data
    WorkModule::set_int(fighter.module_accessor, move_data.startup_frames as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_START_FRAME);
    WorkModule::set_int(fighter.module_accessor, move_data.active_frames as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_ACTIVE_FRAME);
    WorkModule::set_int(fighter.module_accessor, move_data.recovery_frames as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_END_FRAME);
}

fn activate_sandevistan(fighter: &mut L2CFighterCommon) {
    println!("David Mod: Attempting to activate Sandevistan");
    if let Some(sandevistan) = &mut SANDEVISTAN {
        if sandevistan.activate() {
            // Implement Sandevistan effect in SSBU
            // Slow down other fighters
            for fighter_id in 0..8 {
                if fighter_id != Fighter::get_id(fighter.module_accessor) {
                    FighterManager::set_fighter_slow_rate(fighter_id, 0.5);
                }
            }
            // Speed up David
            FighterManager::set_fighter_slow_rate(Fighter::get_id(fighter.module_accessor), 2.0);
            
            // Visual effect
            ColorBlendModule::set_main_color(fighter.module_accessor, &Vector4f{x: 1.0, y: 0.5, z: 0.5, w: 1.0}, 5, 0x50);
            
            // Set duration
            WorkModule::set_int(fighter.module_accessor, 300, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_DURATION);
        }
    }
}

fn use_chrome_overload(fighter: &mut L2CFighterCommon) {
    println!("David Mod: Using Chrome Overload");
    CHROME_BALANCE = (CHROME_BALANCE + 20.0).min(100.0);
    // Implement Chrome Overload effect in SSBU
    AttackModule::set_power_up(fighter.module_accessor, 1.2);
    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);  // Self-damage
    
    // Visual effect
    EffectModule::req_follow(
        fighter.module_accessor,
        Hash40::new("sys_aura_light"),
        Hash40::new("hip"),
        &Vector3f::zero(),
        &Vector3f::zero(),
        1.0,
        true,
        0,
        0,
        0,
        0,
        0,
        true,
        true
    );
}

fn update_chrome_balance(current_balance: f32) -> f32 {
    let new_balance = (current_balance - 0.1).max(0.0);
    println!("David Mod: Updated Chrome Balance from {:.2} to {:.2}", current_balance, new_balance);
    new_balance
}

#[skyline::main(name = "david_mod")]
pub fn main() {
    skyline::install_hooks!(
        david_init,
        david_frame,
        david_move
    );
    init_david();
}