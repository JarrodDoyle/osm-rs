use std::{
    ffi::{CString, c_int},
    os::raw::c_char,
};

use windows::core::*;

use crate::IScriptMan;

#[interface("2A000228-7CA8-13F7-8348-00AA00A82B51")]
unsafe trait IVersionService: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn GetAppName(&self, title_only: BOOL, app_name: &mut *mut c_char);
    fn GetVersion(&self, major: &mut c_int, minor: &mut c_int);
    fn IsEditor(&self) -> c_int;
    // TODO:
}

#[interface("D70000D7-7B57-12A6-8348-00AA00A82B51")]
unsafe trait IDebugService: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn MPrint(
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
    fn Command(
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
    fn Break(&self) -> HRESULT;
    fn Log(
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

fn get_service<T: Interface>(script_manager: &IScriptMan) -> T {
    unsafe { script_manager.GetService(&T::IID).cast::<T>().unwrap() }
}

static mut SERVICES: Option<&Services> = None;

pub struct Services {
    pub debug: DebugService,
    pub version: VersionService,
}

pub(crate) fn services_init(script_manager: IScriptMan) {
    let services = Services {
        debug: DebugService {
            service: get_service(&script_manager),
        },
        version: VersionService {
            service: get_service(&script_manager),
        },
    };
    unsafe { SERVICES = Some(Box::leak(Box::new(services))) };
}

pub fn services() -> &'static Services {
    unsafe { SERVICES.expect("Services hasn't been initialised.") }
}

pub struct DebugService {
    service: IDebugService,
}

impl DebugService {
    pub fn print(&self, msg: &str) {
        let s1 = CString::new(msg).unwrap();
        let s = CString::from(c"");
        unsafe {
            let _ = self.service.MPrint(
                &mut s1.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
            );
        }
    }

    pub fn command(&self, cmd: &str) {
        let s1 = CString::new(cmd).unwrap();
        let s = CString::from(c"");
        unsafe {
            let _ = self.service.Command(
                &mut s1.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
            );
        }
    }

    pub fn log(&self, msg: &str) {
        let s1 = CString::new(msg).unwrap();
        let s = CString::from(c"");
        unsafe {
            let _ = self.service.Log(
                &mut s1.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
                &mut s.as_ptr(),
            );
        }
    }

    pub fn breakpoint(&self) {
        let _ = unsafe { self.service.Break() };
    }
}

pub struct VersionService {
    service: IVersionService,
}

impl VersionService {
    pub fn get_app_name(&self, title_only: bool) -> String {
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.GetAppName(title_only.into(), &mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }

    pub fn get_version(&self) -> (i32, i32) {
        let mut major = 0;
        let mut minor = 0;
        unsafe { self.service.GetVersion(&mut major, &mut minor) };
        (major, minor)
    }

    pub fn is_editor(&self) -> i32 {
        unsafe { self.service.IsEditor() }
    }
}
