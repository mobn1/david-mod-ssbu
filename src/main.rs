use skyline::nro::NroHookBuilder;
use smash::lib::lua_const::*;

mod ssbu_mod;
mod character;
mod moves;
mod sandevistan;

#[skyline::main(name = "david_mod")]
pub fn main() {
    println!("David Mod: Loaded");
    
    NroHookBuilder::new("common")
        .after_init(ssbu_mod::david_init)
        .before_frame(ssbu_mod::david_frame)
        .add_hook(ssbu_mod::david_move)
        .build()
        .unwrap();
}
