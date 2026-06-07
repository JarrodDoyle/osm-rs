use std::{
    ffi::{c_char, c_void},
    result::Result,
};

use kc_osm::*;

pub extern "C" fn log_cmds() {
    let Some(debug) = &services().debug else {
        return;
    };

    for cmd in get_all_command_infos() {
        debug.print(&format!("Command: {cmd}"));
    }
}

pub extern "C" fn echo(val: *const c_char) {
    let Some(debug) = &services().debug else {
        return;
    };

    let msg = cstr_convert(val);
    debug.print(&msg);
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(module: &mut ScriptModule) -> Result<(), String> {
    module
        .register_commands(&[
            Command::new(
                "log_cmds",
                "Output more detailed information for every command in dromed",
                CommandType::FuncVoid,
                CommandContext::EDITOR,
                log_cmds as *const c_void,
            ),
            Command::new(
                "jecho",
                "Prints whatever the input string was to mono",
                CommandType::FuncString,
                CommandContext::ALL,
                echo as *const c_void,
            ),
        ])
        .map_err(|e| format!("{e}"))
}
