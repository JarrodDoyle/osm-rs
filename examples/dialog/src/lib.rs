use std::{
    ffi::{c_char, c_float, c_int, c_void},
    mem::offset_of,
    result::Result,
};

use kc_osm::{
    dialogs::{
        EdittableStruct, FieldDesc, FieldType, StructDesc, StructEditorDesc,
        construct_struct_editor, do_simple_menu,
    },
    *,
};

pub extern "C" fn dialog() {
    let items = vec!["String 1", "lorem", "ipsum", "cool right?"];
    if let Some(idx) = do_simple_menu("Simple Menu Dialog", &items) {
        services()
            .debug
            .print(&format!("Selected '{}' at index {idx}", items[idx]));
    } else {
        services().debug.print("Cancelled");
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ManualStruct {
    float: c_float,
    int: c_int,
}

pub extern "C" fn manual_struct_editor() {
    let mut item = ManualStruct { float: 0.0, int: 2 };

    let editor_desc = StructEditorDesc::new("My Struct Editor", 0);
    let field_descs = [
        FieldDesc::new(
            "Float",
            FieldType::Float,
            size_of::<c_float>(),
            offset_of!(ManualStruct, float),
        ),
        FieldDesc::new("Int", FieldType::Int, 4, offset_of!(ManualStruct, int)),
    ];
    let struct_desc = StructDesc::new(
        "MyStruct",
        size_of::<ManualStruct>() as u32,
        0,
        &field_descs,
    );

    let item_ptr = &mut item as *mut _ as *mut c_void;
    let editor = construct_struct_editor(&editor_desc, &struct_desc, item_ptr);
    if editor.go(true) {
        services().debug.print(&format!("{item:?}"));
    } else {
        services().debug.print("Cancelled");
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, EdittableStruct)]
pub struct AutoStruct {
    #[field_desc]
    float: c_float,
    #[field_desc]
    my_bool: BOOL,
    #[field_desc]
    my_int: c_int,
    #[field_desc]
    my_string: [c_char; 16],
}

pub extern "C" fn auto_struct_editor() {
    let mut item = AutoStruct {
        float: 0.0,
        my_bool: true.into(),
        my_int: 18,
        my_string: [0; 16],
    };

    if item.edit_struct() {
        services().debug.print(&format!("{item:?}"));
    } else {
        services().debug.print("Cancelled");
    }
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(module: &mut ScriptModule) -> Result<(), String> {
    module
        .register_commands(&[
            Command::new(
                "dialog",
                "Displays a list selection menu with DarkDlgs",
                CommandType::FuncVoid,
                CommandContext::EDITOR,
                dialog as *const c_void,
            ),
            Command::new(
                "auto_seditor",
                "Displays a struct editor with DarkDlgs",
                CommandType::FuncVoid,
                CommandContext::EDITOR,
                auto_struct_editor as *const c_void,
            ),
            Command::new(
                "manual_seditor",
                "Displays a manually crafter struct editor with DarkDlgs",
                CommandType::FuncVoid,
                CommandContext::EDITOR,
                manual_struct_editor as *const c_void,
            ),
        ])
        .map_err(|e| format!("{e}"))
}
