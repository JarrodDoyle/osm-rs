use std::ffi::{CString, c_char, c_int, c_uint, c_ulong, c_void};

use libloading::os::windows::Library;
pub use windows::core::*;

#[interface("BE0000BE-7B3E-128D-8348-00AA00A82B51")]
unsafe trait IStructEditor: IUnknown {
    fn Go(&self, modal: c_ulong) -> BOOL;
    // fn SetCallback(&self, )
    // TODO: Implement the other bits :)
}

pub struct StructEditor {
    ed: IStructEditor,
}

impl StructEditor {
    pub fn go(&self, modal: bool) -> bool {
        unsafe { self.ed.Go(if modal { 0 } else { 1 }).as_bool() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StructEditorDesc {
    title: [c_char; 32],
    flags: c_ulong,
}

impl StructEditorDesc {
    pub fn new(title: &str, flags: c_ulong) -> Self {
        let mut title_arr: [c_char; 32] = [0; 32];

        let title = CString::new(title).unwrap();
        let bytes = title.as_bytes();
        for (&x, p) in bytes.iter().zip(title_arr.iter_mut()) {
            *p = x as i8;
        }

        Self {
            title: title_arr,
            flags,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StructDesc {
    name: [c_char; 32],
    size: c_ulong,
    flags: c_uint, // TODO: Enum
    field_count: c_int,
    fields: *const FieldDesc,
}

impl StructDesc {
    pub fn new(name: &str, size: u32, flags: u32, fields: &[FieldDesc]) -> Self {
        let mut name_arr: [c_char; 32] = [0; 32];

        let name = CString::new(name).unwrap();
        let bytes = name.as_bytes();
        for (&x, p) in bytes.iter().zip(name_arr.iter_mut()) {
            *p = x as i8;
        }

        let field_count = fields.len() as c_int;
        let fields = Box::leak(Box::new(fields.to_vec())).as_ptr();

        Self {
            name: name_arr,
            size,
            flags,
            field_count,
            fields,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FieldDesc {
    name: [c_char; 32],
    field_type: c_uint, // TODO: Enum
    size: c_ulong,
    offset: c_ulong,
    flags: c_uint, // TODO: Enum,
    min_value: c_int,
    max_value: c_int,
    element_count: c_ulong,
    elements: *mut c_void,
}

impl FieldDesc {
    pub fn new(name: &str, field_type: u32, size: u32, offset: u32) -> Self {
        let mut name_arr: [c_char; 32] = [0; 32];
        let name = CString::new(name).unwrap();
        let bytes = name.as_bytes();
        for (&x, p) in bytes.iter().zip(name_arr.iter_mut()) {
            *p = x as i8;
        }

        Self {
            name: name_arr,
            field_type,
            size,
            offset,
            ..Default::default()
        }
    }
}

pub fn do_simple_menu(title: &str, items: &[&str]) -> Option<usize> {
    let func = unsafe {
        Library::new("darkdlgs")
            .unwrap()
            .get::<unsafe extern "stdcall" fn(*const c_char, *const *const c_char, c_int) -> c_int>(
                b"_DoSimpleMenu@12",
            )
            .unwrap()
    };

    let title = CString::new(title).unwrap();
    let mut cstrings = vec![];
    let mut list = vec![];
    for item in items {
        cstrings.push(CString::new(*item).unwrap());
        list.push(cstrings.last().unwrap().as_ptr());
    }
    let idx = unsafe { func(title.as_ptr(), list.as_ptr(), items.len() as c_int) };
    if idx < 0 { None } else { Some(idx as usize) }
}

pub fn construct_struct_editor(
    editor_desc: &StructEditorDesc,
    struct_desc: &StructDesc,
    item: *mut c_void,
) -> StructEditor {
    let func = unsafe {
        Library::new("darkdlgs")
            .unwrap()
            .get::<unsafe extern "stdcall" fn(
                *const StructEditorDesc,
                *const StructDesc,
                *const c_void,
            ) -> IStructEditor>(b"_ConstructStructEditor@12")
            .unwrap()
    };

    let ed = unsafe { func(editor_desc, struct_desc, item) };
    StructEditor { ed }
}
