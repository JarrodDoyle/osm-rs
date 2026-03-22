use std::{
    ffi::{CString, c_char, c_int, c_ulong, c_void},
    fmt::Display,
    ptr::null,
    sync::{LazyLock, Mutex},
};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::cstr_convert;

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

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub type_: CommandType,
    pub comment: String,
    pub contexts: u32,
    pub unknown: i32,
}

impl Display for CommandInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "('{}', {:?}, '{}', {}, {})",
            self.name, self.type_, self.comment, self.contexts, self.unknown
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub name: *const c_char,
    pub type_: CommandType,
    pub val: *const c_void,
    pub comment: *const c_char,
    pub contexts: c_ulong,
    pub unknown: c_int,
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

fn get_command_ptrs() -> (*mut c_int, *mut *const Command, *mut c_int) {
    let base = unsafe { GetModuleHandleA(null()) } as u32;
    let command_list_size_ptr = (base + 0x6809bc) as *mut c_int;
    let command_list_ptr = (base + 0x6809c0) as *mut *const Command;
    let command_count_ptr = (base + 0x680dc0) as *mut c_int;
    (command_list_size_ptr, command_list_ptr, command_count_ptr)
}

pub fn get_all_command_infos<'a>() -> Vec<CommandInfo> {
    let mut commands = vec![];
    let (command_list_size_ptr, command_list_ptr, command_count_ptr) = get_command_ptrs();
    unsafe {
        for i in 0..*command_list_size_ptr {
            let count = *command_count_ptr.offset(i as isize);
            for j in 0..count {
                let cmd = *(*command_list_ptr.offset(i as isize)).offset(j as isize);
                commands.push(CommandInfo {
                    name: cstr_convert(cmd.name),
                    type_: cmd.type_,
                    comment: cstr_convert(cmd.comment),
                    contexts: cmd.contexts,
                    unknown: cmd.unknown,
                });
            }
        }
    };

    commands
}

pub(crate) fn register_command_set(cmds: &[Command]) {
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

    let mut commands = COMMAND_SETS.lock().unwrap();
    commands.push(cmds_ptr as usize);
}

pub(crate) fn deregister_command_sets() {
    let commands = COMMAND_SETS.lock().unwrap();
    let (command_list_size_ptr, command_list_ptr, command_count_ptr) = get_command_ptrs();
    unsafe {
        for cmd_set in commands.iter() {
            let cmd_set_ptr = *cmd_set as *const Command;
            let mut found = false;
            for i in 0..*command_list_size_ptr {
                if *command_list_ptr.offset(i as isize) == cmd_set_ptr {
                    found = true;
                }

                if !found {
                    continue;
                }

                // Shift everything afterwards down
                *command_list_ptr.offset(i as isize) = *command_list_ptr.offset((i + 1) as isize);
                *command_count_ptr.offset(i as isize) = *command_count_ptr.offset((i + 1) as isize);
                *command_list_size_ptr -= 1;
            }
        }
    };
}

static COMMAND_SETS: LazyLock<Mutex<Vec<usize>>> = LazyLock::new(|| Mutex::new(vec![]));
