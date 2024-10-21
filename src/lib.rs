#![feature(proc_macro_hygiene)]

use smash::lua2cpp::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::*;
use smashline::*;

use nalgebra as na;
use std::sync::Mutex;

mod character;
mod moves;
mod sandevistan;

use character::David;
use moves::{Move, get_move_data};
use sandevistan::Sandevistan;

lazy_static::lazy_static! {
    static ref DAVID: Mutex<Option<David>> = Mutex::new(None);
    static ref SANDEVISTAN: Mutex<Option<Sandevistan>> = Mutex::new(None);
    static ref CHROME_BALANCE: Mutex<f32> = Mutex::new(50.0);
}

#[fighter_init]
fn david_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
            let mut david = DAVID.lock().unwrap();
            *david = Some(David::new());
            let mut sandevistan = SANDEVISTAN.lock().unwrap();
            *sandevistan = Some(Sandevistan::new());
            let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
            *chrome_balance = 50.0;
            println!("David Mod: David's initial state set");
        }
    }
}

#[fighter_frame]
fn david_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
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
    }
}

#[fighter_reset]
fn david_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_MARIO {  // Replace Mario with David
            let mut david = DAVID.lock().unwrap();
            *david = Some(David::new());
            let mut sandevistan = SANDEVISTAN.lock().unwrap();
            *sandevistan = Some(Sandevistan::new());
            let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
            *chrome_balance = 50.0;
            println!("David Mod: David's state reset");
        }
    }
}

#[acmd_script(agent = "mario", script = "game_attack11", category = "game")]
fn david_jab(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::Jab);
}

#[acmd_script(agent = "mario", script = "game_attackdash", category = "game")]
fn david_dash_attack(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::DashAttack);
}

#[acmd_script(agent = "mario", script = "game_attacks3", category = "game")]
fn david_ftilt(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::TiltSide);
}

#[acmd_script(agent = "mario", script = "game_attackhi3", category = "game")]
fn david_utilt(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::TiltUp);
}

#[acmd_script(agent = "mario", script = "game_attacklw3", category = "game")]
fn david_dtilt(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::TiltDown);
}

#[acmd_script(agent = "mario", script = "game_attacks4", category = "game")]
fn david_fsmash(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::SmashSide);
}

#[acmd_script(agent = "mario", script = "game_attackhi4", category = "game")]
fn david_usmash(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::SmashUp);
}

#[acmd_script(agent = "mario", script = "game_attacklw4", category = "game")]
fn david_dsmash(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::SmashDown);
}

#[acmd_script(agent = "mario", script = "game_specialn", category = "game")]
fn david_neutral_special(fighter: &mut L2CAgentBase) {
    activate_sandevistan(fighter);
}

#[acmd_script(agent = "mario", script = "game_specials", category = "game")]
fn david_side_special(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::SpecialSide);
}

#[acmd_script(agent = "mario", script = "game_specialhi", category = "game")]
fn david_up_special(fighter: &mut L2CAgentBase) {
    execute_move(fighter, Move::SpecialUp);
}

#[acmd_script(agent = "mario", script = "game_speciallw", category = "game")]
fn david_down_special(fighter: &mut L2CAgentBase) {
    use_chrome_overload(fighter);
}

fn execute_move(fighter: &mut L2CAgentBase, move_type: Move) {
    let move_data = get_move_data(&move_type);
    let mut david = DAVID.lock().unwrap();
    if let Some(d) = david.as_mut() {
        d.apply_force(move_data.knockback.x);
    }
    
    acmd!(fighter, {
        frame(Frame=move_data.startup_frames)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=move_data.damage, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=move_data.active_frames)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

fn activate_sandevistan(fighter: &mut L2CAgentBase) {
    let mut sandevistan = SANDEVISTAN.lock().unwrap();
    if let Some(s) = sandevistan.as_mut() {
        if s.activate() {
            acmd!(fighter, {
                if(is_excute){
                    SlowModule::set_whole(8, 0)
                    EffectModule::req_follow(hash40("sys_aura_light"), hash40("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0x40000000 as u32, 0, 0, 0, 0, true, true)
                }
                frame(Frame=300)
                if(is_excute){
                    SlowModule::clear_whole()
                    EffectModule::kill_kind(hash40("sys_aura_light"), false, true)
                }
            });
        }
    }
}

fn use_chrome_overload(fighter: &mut L2CAgentBase) {
    let mut chrome_balance = CHROME_BALANCE.lock().unwrap();
    *chrome_balance = (*chrome_balance + 20.0).min(100.0);
    acmd!(fighter, {
        if(is_excute){
            AttackModule::set_power_up(1.2)
            DamageModule::add_damage(5.0, 0)
            EffectModule::req_follow(hash40("sys_aura_light"), hash40("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0x40000000 as u32, 0, 0, 0, 0, true, true)
        }
        frame(Frame=30)
        if(is_excute){
            AttackModule::set_power_up(1.0)
            EffectModule::kill_kind(hash40("sys_aura_light"), false, true)
        }
    });
}

fn update_chrome_balance(current_balance: f32) -> f32 {
    (current_balance - 0.1).max(0.0)
}

#[no_mangle]
pub extern "C" fn get_david_state() -> *mut std::os::raw::c_char {
    let state = format!("David's state: Position: {:?}, Sandevistan: {}, Chrome Balance: {:.2}",
        DAVID.lock().unwrap().as_ref().map(|d| d.position),
        SANDEVISTAN.lock().unwrap().as_ref().map(|s| s.is_active()).unwrap_or(false),
        *CHROME_BALANCE.lock().unwrap()
    );
    let c_str = std::ffi::CString::new(state).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_david_state(s: *mut std::os::raw::c_char) {
    unsafe {
        drop(std::ffi::CString::from_raw(s));
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(david_init);
    smashline::install_agent_frames!(david_frame);
    smashline::install_agent_resets!(david_reset);
    smashline::install_acmd_scripts!(
        david_jab,
        david_dash_attack,
        david_ftilt,
        david_utilt,
        david_dtilt,
        david_fsmash,
        david_usmash,
        david_dsmash,
        david_neutral_special,
        david_side_special,
        david_up_special,
        david_down_special
    );
}