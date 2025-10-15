#[unsafe(no_mangle)]
unsafe extern "stdcall" fn ScriptModuleInit(
    _: *const (),
    _: *mut (),
    _: *mut (),
    _: *mut (),
    _: *mut *mut (),
) -> i32 {
    1
}
