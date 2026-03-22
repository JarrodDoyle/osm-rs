use std::{
    ffi::{CString, c_char, c_int, c_ulong, c_void},
    mem::transmute,
    ptr::null,
    result::Result,
};

use kc_osm::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub name: *const c_char,
    pub type_: c_int,
    pub val: *const c_void,
    pub comment: *const c_char,
    pub contexts: c_ulong,
    unknown: c_int,
}

pub fn build_command(
    name: &str,
    help: &str,
    type_: i32,
    contexts: u32,
    val: *const c_void,
) -> Command {
    Command {
        name: CString::new(name).unwrap().into_raw(),
        type_,
        val,
        comment: CString::new(help).unwrap().into_raw(),
        contexts,
        unknown: 0,
    }
}

pub fn register_command_set(cmds_ptr: *const Command, count: c_int) {
    if count <= 0 {
        return;
    }

    const COMMAND_LIST_SIZE_OFFSET: u32 = 0x6809bc;
    const COMMAND_LIST_OFFSET: u32 = 0x6809c0;
    const COMMAND_COUNT_OFFSET: u32 = 0x680dc0;
    let base = unsafe { GetModuleHandleA(null()) } as u32;

    let command_list_size_ptr: *mut c_int = unsafe { transmute(base + COMMAND_LIST_SIZE_OFFSET) };
    let command_list_ptr: *mut *const Command = unsafe { transmute(base + COMMAND_LIST_OFFSET) };
    let command_count_ptr: *mut c_int = unsafe { transmute(base + COMMAND_COUNT_OFFSET) };

    unsafe {
        println!("Command list size: {}", *command_list_size_ptr);
        let size_offset = (*command_list_size_ptr) as isize;
        let set_ptr = command_list_ptr.offset(size_offset);
        *set_ptr = cmds_ptr;
        let count_ptr = command_count_ptr.offset(size_offset);
        *count_ptr = count;
        *command_list_size_ptr = *command_list_size_ptr + 1;
        println!("Command list size: {}", *command_list_size_ptr);

        let cmd = *cmds_ptr;
        println!("Registered Cmd: {:?}", cmd);
    };
}

pub extern "C" fn print_shit() {
    let services = services();
    services.debug.print("Wow this is my custom command.");
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(_: &mut ScriptModule) -> Result<(), &'static str> {
    let cmds = Box::new([
        build_command(
            "jay_custom_command",
            "This is a custom command that does *something* epic",
            0,
            0xffffffff,
            print_shit as *const c_void,
        ),
        build_command(
            "jay_custom_command2",
            "seccond command!",
            0,
            0xffffffff,
            print_shit as *const c_void,
        ),
    ]);
    let cmds_ptr = Box::into_raw(cmds) as *const Command;

    register_command_set(cmds_ptr, 2);

    println!("shit head");
    Ok(())
}
