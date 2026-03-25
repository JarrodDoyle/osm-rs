use std::{
    ffi::{c_float, c_int, c_void},
    result::Result,
};

use kc_osm::{
    dialogs::{
        FieldDesc, FieldType, StructDesc, StructEditorDesc, construct_struct_editor, do_simple_menu,
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
pub struct MyStruct {
    float: c_float,
    int: c_int,
}

pub extern "C" fn struct_editor() {
    let editor_desc = StructEditorDesc::new("My Struct Editor", 0);
    let field_descs = [
        FieldDesc::new("Float", FieldType::Float, 4, 0),
        FieldDesc::new("Int", FieldType::Int, 4, 4),
    ];
    let struct_desc = StructDesc::new("MyStruct", 8, 0, &field_descs);

    let item = MyStruct { float: 0.0, int: 2 };
    let item_ptr = Box::into_raw(Box::new(item)) as *mut c_void;
    let editor = construct_struct_editor(&editor_desc, &struct_desc, item_ptr);
    if editor.go(true) {
        let item = unsafe { Box::from_raw(item_ptr as *mut MyStruct) };
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
                "seditor",
                "Displays a struct editor with DarkDlgs",
                CommandType::FuncVoid,
                CommandContext::EDITOR,
                struct_editor as *const c_void,
            ),
        ])
        .map_err(|e| format!("{e}"))
}
