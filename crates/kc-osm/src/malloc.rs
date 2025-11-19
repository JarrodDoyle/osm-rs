use std::ffi::c_void;

use windows::Win32::System::Com::IMalloc;

static mut MALLOC: Option<&IMalloc> = None;

fn malloc() -> &'static IMalloc {
    unsafe { MALLOC.expect("Malloc hasn't been initialised.") }
}

pub(crate) fn init(malloc: IMalloc) {
    unsafe { MALLOC = Some(Box::leak(Box::new(malloc))) };
}

pub(crate) unsafe fn alloc(size: usize) {
    unsafe { malloc().Alloc(size) };
}

/// # Safety
///
/// `ptr` must point to memory allocated by `malloc::alloc` or Dark Engine
pub(crate) unsafe fn free(ptr: *const c_void) {
    unsafe { malloc().Free(Some(ptr)) };
}
