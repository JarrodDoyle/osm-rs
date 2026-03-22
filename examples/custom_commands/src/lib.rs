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
    if val.is_null() {
        return "".to_owned();
    }
    unsafe { CStr::from_ptr(val).to_string_lossy().into_owned() }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum CommandType {
    FuncVoid,
    FuncBool,
    FuncInt,
    FuncFloat,
    FuncDouble,
    FuncString,
    VarBool,
    VarInt,
    VarString,
    VarIntArray,
    VarFloat,
    ToggleBool,
    ToggleInt,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub name: *const c_char,
    pub type_: CommandType,
    pub val: *const c_void,
    pub comment: *const c_char,
    pub contexts: c_ulong,
    unknown: c_int,
}

impl Command {
    pub fn new(
        name: &str,
        help: &str,
        type_: CommandType,
        contexts: u32,
        func: *const c_void,
    ) -> Self {
        Self {
            name: CString::new(name).unwrap().into_raw(),
            type_,
            val: func,
            comment: CString::new(help).unwrap().into_raw(),
            contexts,
            unknown: 0,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = cstr_convert(self.name);
        let comment = cstr_convert(self.comment);
        write!(
            f,
            "('{}', {:?}, fn_ptr: {:?}, '{}', {}, {})",
            name, self.type_, self.val, comment, self.contexts, self.unknown
        )
    }
}

pub fn get_command_ptrs() -> (*mut c_int, *mut *const Command, *mut c_int) {
    let base = unsafe { GetModuleHandleA(null()) } as u32;
    let command_list_size_ptr = (base + 0x6809bc) as *mut c_int;
    let command_list_ptr = (base + 0x6809c0) as *mut *const Command;
    let command_count_ptr = (base + 0x680dc0) as *mut c_int;
    (command_list_size_ptr, command_list_ptr, command_count_ptr)
}

pub fn register_command_set(cmds: &[Command]) {
    let count = cmds.len() as i32;
    if count <= 0 {
        return;
    }

    let cmds_ptr = Box::leak(Box::new(cmds.to_vec())).as_ptr();
    let (command_list_size_ptr, command_list_ptr, command_count_ptr) = get_command_ptrs();
    unsafe {
        let size_offset = (*command_list_size_ptr) as isize;
        *command_list_ptr.offset(size_offset) = cmds_ptr;
        *command_count_ptr.offset(size_offset) = count;
        *command_list_size_ptr += 1;
    };
}

pub extern "C" fn log_cmds() {
    let debug = &services().debug;
    let (command_list_size_ptr, command_list_ptr, command_count_ptr) = get_command_ptrs();
    unsafe {
        for i in 0..*command_list_size_ptr {
            let count = *command_count_ptr.offset(i as isize);
            debug.print(&format!("Command set contains {count} commands"));
            for j in 0..count {
                let set_ptr = command_list_ptr.offset(i as isize);
                let cmd = *(*set_ptr).offset(j as isize);
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
        Command::new(
            "jay_custom_command",
            "This is a custom command that does *something* epic",
            CommandType::FuncVoid,
            0xffffffff,
            print_shit as *const c_void,
        ),
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
