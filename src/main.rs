use std::ffi::CStr;

// This file is no longer needed as the main entry point.
// The plugin.rs file now serves as the entry point for our mod.
// We'll keep this file for potential future use or testing purposes.

#[allow(dead_code)]
fn main() {
    println!("David Mod: Main function (for testing purposes)");
    
    // Example of how to use the get_david_state function
    extern "C" {
        fn get_david_state() -> *const std::os::raw::c_char;
        fn free_david_state(s: *mut std::os::raw::c_char);
    }

    unsafe {
        let state_ptr = get_david_state();
        let state = CStr::from_ptr(state_ptr).to_string_lossy().into_owned();
        println!("Current David state: {}", state);
        free_david_state(state_ptr as *mut std::os::raw::c_char);
    }
}
