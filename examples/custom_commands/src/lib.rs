use std::{
    ffi::{c_char, c_void},
    result::Result,
};

use kc_osm::*;

pub extern "C" fn log_cmds() {
    let debug = &services().debug;
    for cmd in get_all_command_infos() {
        debug.print(&format!("Command: {cmd}"));
    }
}

pub extern "C" fn echo(val: *const c_char) {
    let msg = cstr_convert(val);
    services().debug.print(&msg);
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(module: &mut ScriptModule) -> Result<(), &'static str> {
    module.register_commands(&[
        Command::new(
            "log_cmds",
            "Output more detailed information for every command in dromed",
            CommandType::FuncVoid,
            0xffffffff,
            log_cmds as *const c_void,
        ),
        Command::new(
            "jecho",
            "Prints whatever the input string was to mono",
            CommandType::FuncString,
            0xffffffff,
            echo as *const c_void,
        ),
    ]);

    Ok(())
}
