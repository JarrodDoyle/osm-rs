use std::{
    ffi::{CStr, CString, c_char, c_int, c_ulong, c_void},
    fmt::Display,
    mem::transmute,
    ptr::null,
    result::Result,
};

use kc_osm::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;

fn cstr_convert(val: *const c_char) -> String {
    if val == null() {
        return "".to_owned();
    }
    unsafe { CStr::from_ptr(val).to_string_lossy().into_owned() }
}

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

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = cstr_convert(self.name);
        let comment = cstr_convert(self.comment);
        write!(
            f,
            "('{}', {}, fn_ptr: {:?}, '{}', {}, {})",
            name, self.type_, self.val, comment, self.contexts, self.unknown
        )
    }
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

pub fn register_command_set(cmds: &[Command]) {
    let count = cmds.len() as i32;
    if count <= 0 {
        return;
    }

    let cmds_ptr = Box::leak(Box::new(cmds.to_vec())).as_ptr();

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

pub extern "C" fn log_cmds() {
    let debug = &services().debug;

    const COMMAND_LIST_SIZE_OFFSET: u32 = 0x6809bc;
    const COMMAND_LIST_OFFSET: u32 = 0x6809c0;
    const COMMAND_COUNT_OFFSET: u32 = 0x680dc0;
    let base = unsafe { GetModuleHandleA(null()) } as u32;

    let command_list_size_ptr: *mut c_int = unsafe { transmute(base + COMMAND_LIST_SIZE_OFFSET) };
    let command_list_ptr: *mut *const Command = unsafe { transmute(base + COMMAND_LIST_OFFSET) };
    let command_count_ptr: *mut c_int = unsafe { transmute(base + COMMAND_COUNT_OFFSET) };

    unsafe {
        for i in 0..(*command_list_size_ptr) {
            let count = *(command_count_ptr.offset(i as isize));
            debug.print(&format!("Command set contains {count} commands"));
            for j in 0..count {
                let set_ptr = command_list_ptr.offset(i as isize);
                let cmd = *((*set_ptr).offset(j as isize));
                debug.print(&format!("Command: {cmd}"));
            }
        }
    };
}

pub extern "C" fn print_shit() {
    let services = services();
    services.debug.print("Wow this is my custom command.");
}

pub extern "C" fn echo(val: *const c_char) {
    let msg = cstr_convert(val);
    services().debug.print(&msg);
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(_: &mut ScriptModule) -> Result<(), &'static str> {
    register_command_set(&[
        build_command(
            "jay_custom_command",
            "This is a custom command that does *something* epic",
            0,
            0xffffffff,
            print_shit as *const c_void,
        ),
        build_command(
            "log_cmds",
            "Output more detailed information for every command in dromed",
            0,
            0xffffffff,
            log_cmds as *const c_void,
        ),
        build_command(
            "jecho",
            "Prints whatever the input string was to mono",
            5,
            0xffffffff,
            echo as *const c_void,
        ),
    ]);

    Ok(())
}
