use arcropolis_api::{self as arc, skyline};
use nro_hook::nro_hook;
use smash::lua2cpp::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::*;

use nalgebra as na;
use std::sync::Mutex;

mod character;
mod moves;
mod sandevistan;

use character::David;
use moves::{Move, get_move_data, MoveData};
use sandevistan::Sandevistan;

lazy_static::lazy_static! {
    static ref DAVID: Mutex<Option<David>> = Mutex::new(None);
    static ref SANDEVISTAN: Mutex<Option<Sandevistan>> = Mutex::new(None);
    static ref CHROME_BALANCE: Mutex<f32> = Mutex::new(50.0);
}

#[skyline::hook(offset = 0x37BAD0)]
pub unsafe fn david_init(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        let mut david = DAVID.lock().unwrap();
        *david = Some(David::new());
        let mut sandevistan = SANDEVISTAN.lock().unwrap();
        *sandevistan = Some(Sandevistan::new());
        let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
        *chrome_balance = 50.0;
        println!("David Mod: David's initial state set");
    }
    call_original!(fighter)
}

#[skyline::hook(offset = 0x37BAE0)]
pub unsafe fn david_frame(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        let mut david = DAVID.lock().unwrap();
        if let Some(d) = david.as_mut() {
            d.update();
        }
        let mut sandevistan = SANDEVISTAN.lock().unwrap();
        if let Some(s) = sandevistan.as_mut() {
            s.update(std::time::Duration::from_millis(16));
        }
        let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
        *chrome_balance = update_chrome_balance(*chrome_balance);
    }
    call_original!(fighter)
}

#[skyline::hook(offset = 0x37BBF0)]
pub unsafe fn david_move(fighter: &mut L2CFighterCommon) {
    if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        match status_kind {
            *FIGHTER_STATUS_KIND_ATTACK => execute_move(fighter, Move::Jab),
            *FIGHTER_STATUS_KIND_ATTACK_DASH => execute_move(fighter, Move::DashAttack),
            *FIGHTER_STATUS_KIND_ATTACK_S3 => execute_move(fighter, Move::TiltSide),
            *FIGHTER_STATUS_KIND_ATTACK_HI3 => execute_move(fighter, Move::TiltUp),
            *FIGHTER_STATUS_KIND_ATTACK_LW3 => execute_move(fighter, Move::TiltDown),
            *FIGHTER_STATUS_KIND_ATTACK_S4 => execute_move(fighter, Move::SmashSide),
            *FIGHTER_STATUS_KIND_ATTACK_HI4 => execute_move(fighter, Move::SmashUp),
            *FIGHTER_STATUS_KIND_ATTACK_LW4 => execute_move(fighter, Move::SmashDown),
            *FIGHTER_STATUS_KIND_SPECIAL_N => activate_sandevistan(fighter),
            *FIGHTER_STATUS_KIND_SPECIAL_S => execute_move(fighter, Move::SpecialSide),
            *FIGHTER_STATUS_KIND_SPECIAL_HI => execute_move(fighter, Move::SpecialUp),
            *FIGHTER_STATUS_KIND_SPECIAL_LW => use_chrome_overload(fighter),
            _ => {}
        }
    }
    call_original!(fighter)
}

fn execute_move(fighter: &mut L2CFighterCommon, move_type: Move) {
    let move_data = get_move_data(&move_type);
    let mut david = DAVID.lock().unwrap();
    if let Some(d) = david.as_mut() {
        d.apply_force(move_data.knockback.x);
    }
    
    DamageModule::add_damage(fighter.module_accessor, move_data.damage, 0);
    let angle = move_data.knockback.y.atan2(move_data.knockback.x);
    let speed = move_data.knockback.magnitude();
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, speed, angle, 0, 0, 0);
    sv_kinetic_energy!(set_speed, fighter);
    
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
}

fn activate_sandevistan(fighter: &mut L2CFighterCommon) {
    let mut sandevistan = SANDEVISTAN.lock().unwrap();
    if let Some(s) = sandevistan.as_mut() {
        if s.activate() {
            FighterManager::set_fighter_slow_rate(Fighter::get_id(fighter.module_accessor), 2.0);
            ColorBlendModule::set_main_color(fighter.module_accessor, &Vector4f{x: 1.0, y: 0.5, z: 0.5, w: 1.0}, 5, 0x50);
            WorkModule::set_int(fighter.module_accessor, 300, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_DURATION);
        }
    }
}

fn use_chrome_overload(fighter: &mut L2CFighterCommon) {
    let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
    *chrome_balance = (*chrome_balance + 20.0).min(100.0);
    AttackModule::set_power_up(fighter.module_accessor, 1.2);
    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);
    
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
    (current_balance - 0.1).max(0.0)
}

#[nro_hook(pattern = "40 55 53 48 8D 2D ? ? ? ? 48 83 EC 28")]
unsafe fn david_mod_main() {
    skyline::install_hooks!(
        david_init,
        david_frame,
        david_move
    );
}
