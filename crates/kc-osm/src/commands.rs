use std::{
    ffi::{CString, c_char, c_int, c_ulong, c_void},
    fmt::Display,
    ptr::null,
    sync::{LazyLock, Mutex},
};

use bitflags::bitflags;
use thiserror::Error;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::{cstr_convert, services};

#[derive(Error, Debug)]
pub enum CommandRegisterError {
    #[error("Cannot register empty command group.")]
    EmptySet,
    #[error("Max command groups reached.")]
    SetLimitReached,
    #[error("Unsupported engine version.")]
    UnsupportedEngineVersion,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandContext: c_ulong {
        const GAME_MODE = 0b00000001;
        const BRUSH_EDIT = 0b00000010;
        const OBJ_EDIT = 0b00000100;
        const GAME_MODE2 = 0b00100000;

        const EDITOR = Self::BRUSH_EDIT.bits() | Self::OBJ_EDIT.bits();
        const ALL = !0;
    }
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

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub type_: CommandType,
    pub comment: String,
    pub contexts: CommandContext,
    pub unknown: i32,
}

impl Display for CommandInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "('{}', {:?}, '{}', {:?}, {})",
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
    pub contexts: CommandContext,
    pub unknown: c_int,
}

impl Command {
    pub fn new(
        name: &str,
        help: &str,
        type_: CommandType,
        contexts: CommandContext,
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

fn get_command_ptrs() -> Option<(*mut c_int, *mut *const Command, *mut c_int)> {
    let version = &services().version;
    let offsets = match (version.get_version(), version.is_editor() != 0) {
        ((1, 19), true) => (0x5f86fc, 0x5f8700, 0x5f8b00),
        ((1, 20), true) => (0x6023bc, 0x6023c0, 0x6027c0),
        ((1, 21), true) => (0x6033bc, 0x6033c0, 0x6037c0),
        ((1, 22), true) => (0x61865c, 0x618660, 0x618a60),
        ((1, 23), true) => (0x6269bc, 0x6269c0, 0x626dc0),
        ((1, 24), true) => (0x6279bc, 0x6279c0, 0x627dc0),
        ((1, 25), true) => (0x629abc, 0x629ac0, 0x629ec0),
        ((1, 26), true) => (0x62a0dc, 0x62a0e0, 0x62a4e0),
        ((1, 27), true) => (0x62e17c, 0x62e180, 0x62e580),
        ((1, 28), true) => (0x6809bc, 0x6809c0, 0x680dc0),
        _ => return None,
    };

    let base = unsafe { GetModuleHandleA(null()) } as u32;
    let set_count_ptr = (base + offsets.0) as *mut c_int;
    let sets_ptr = (base + offsets.1) as *mut *const Command;
    let cmd_count_ptr = (base + offsets.2) as *mut c_int;
    Some((set_count_ptr, sets_ptr, cmd_count_ptr))
}

pub fn get_all_command_infos<'a>() -> Vec<CommandInfo> {
    let mut commands = vec![];
    if let Some((set_count_ptr, sets_ptr, cmd_count_ptr)) = get_command_ptrs() {
        unsafe {
            for i in 0..*set_count_ptr {
                let count = *cmd_count_ptr.offset(i as isize);
                for j in 0..count {
                    let cmd = *(*sets_ptr.offset(i as isize)).offset(j as isize);
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
    }

    commands
}

const MAX_COMMAND_SETS: i32 = 256; // Checked that it's the same across versions in Ghidra
pub(crate) fn register_command_set(cmds: &[Command]) -> Result<(), CommandRegisterError> {
    let count = cmds.len() as i32;
    if count <= 0 {
        return Err(CommandRegisterError::EmptySet);
    }

    if let Some((set_count_ptr, sets_ptr, cmd_count_ptr)) = get_command_ptrs() {
        if unsafe { *set_count_ptr } >= MAX_COMMAND_SETS {
            return Err(CommandRegisterError::SetLimitReached);
        }

        let cmds_ptr = Box::leak(Box::new(cmds.to_vec())).as_ptr();
        unsafe {
            let size_offset = (*set_count_ptr) as isize;
            *sets_ptr.offset(size_offset) = cmds_ptr;
            *cmd_count_ptr.offset(size_offset) = count;
            *set_count_ptr += 1;
        };

        COMMAND_SETS.lock().unwrap().push(cmds_ptr as usize);
        return Ok(());
    }

    return Err(CommandRegisterError::UnsupportedEngineVersion);
}

pub(crate) fn deregister_command_sets() {
    if let Some((set_count_ptr, sets_ptr, cmd_count_ptr)) = get_command_ptrs() {
        unsafe {
            for cmd_set in COMMAND_SETS.lock().unwrap().iter() {
                let cmd_set_ptr = *cmd_set as *const Command;
                let mut found = false;
                for i in 0..*set_count_ptr {
                    if *sets_ptr.offset(i as isize) == cmd_set_ptr {
                        found = true;
                        *set_count_ptr -= 1;
                    }

                    if !found {
                        continue;
                    }

                    // Shift everything afterwards down
                    *sets_ptr.offset(i as isize) = *sets_ptr.offset((i + 1) as isize);
                    *cmd_count_ptr.offset(i as isize) = *cmd_count_ptr.offset((i + 1) as isize);
                }
            }
        };
    }
}

static COMMAND_SETS: LazyLock<Mutex<Vec<usize>>> = LazyLock::new(|| Mutex::new(vec![]));
