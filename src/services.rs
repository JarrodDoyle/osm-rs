use std::{ffi::c_int, os::raw::c_char};

use windows::core::*;

#[interface("2A000228-7CA8-13F7-8348-00AA00A82B51")]
pub unsafe trait IVersionService: IUnknown {
    pub fn Init(&self);
    pub fn End(&self);
    pub fn GetAppName(&self, title_only: BOOL, app_name: &mut *mut c_char);
    pub fn GetVersion(&self, major: &mut c_int, minor: &mut c_int);
    pub fn IsEditor(&self) -> c_int;
    // TODO:
}

#[interface("D70000D7-7B57-12A6-8348-00AA00A82B51")]
pub unsafe trait IDebugService: IUnknown {
    pub fn Init(&self);
    pub fn End(&self);
    pub fn MPrint(
        &self,
        s1: &mut *const c_char,
        s2: &mut *const c_char,
        s3: &mut *const c_char,
        s4: &mut *const c_char,
        s5: &mut *const c_char,
        s6: &mut *const c_char,
        s7: &mut *const c_char,
        s8: &mut *const c_char,
    ) -> HRESULT;
    pub fn Command(
        &self,
        s1: &mut *const c_char,
        s2: &mut *const c_char,
        s3: &mut *const c_char,
        s4: &mut *const c_char,
        s5: &mut *const c_char,
        s6: &mut *const c_char,
        s7: &mut *const c_char,
        s8: &mut *const c_char,
    ) -> HRESULT;
    pub fn Break(&self) -> HRESULT;
    pub fn Log(
        &self,
        s1: &mut *const c_char,
        s2: &mut *const c_char,
        s3: &mut *const c_char,
        s4: &mut *const c_char,
        s5: &mut *const c_char,
        s6: &mut *const c_char,
        s7: &mut *const c_char,
        s8: &mut *const c_char,
    ) -> HRESULT;
}
