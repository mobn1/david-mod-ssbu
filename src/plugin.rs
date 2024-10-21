#![feature(proc_macro_hygiene)]

use skyline::nro::NroInfo;
use skyline::hooks::InlineCtx;

#[skyline::main(name = "david_mod")]
pub fn main() {
    println!("David Mod: Plugin loaded");
    david_mod::install();
}

#[skyline::hook(offset = 0x3756240, inline)]
pub fn handle_arc_file(_ctx: &InlineCtx, file_info: &mut arcropolis_api::FileInfo) {
    if file_info.path.contains("fighter/mario") {
        println!("David Mod: Intercepted Mario file, replacing with David");
        // Here you would implement the logic to replace Mario's files with David's
        // This could involve changing file paths, modifying file contents, etc.
    }
}

#[skyline::from_offset(0x3756240)]
pub fn arc_callback_original(file_info: &mut arcropolis_api::FileInfo);

pub fn arc_callback(file_info: &mut arcropolis_api::FileInfo) {
    handle_arc_file(&InlineCtx::new(), file_info);
    unsafe {
        arc_callback_original(file_info);
    }
}

#[skyline::from_offset(0x37563F0)]
pub fn nro_load_original(nro: &skyline::nro::NroInfo) -> u32;

#[skyline::hook(replace = nro_load_original)]
pub fn nro_load_hook(nro: &NroInfo) -> u32 {
    let ret = nro_load_original(nro);
    if nro.name == "common" {
        skyline::install_hook!(arc_callback);
    }
    ret
}
