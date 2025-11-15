use std::{
    ffi::{CStr, CString, c_float, c_int},
    os::raw::c_char,
    ptr::null,
    str::FromStr,
};

use windows::core::*;

use crate::{IScriptMan, sMultiParm};

static mut SERVICES: Option<&Services> = None;

pub struct Services {
    pub act_react: ActReactService,
    pub debug: DebugService,
    pub version: VersionService,
}

pub fn services() -> &'static Services {
    unsafe { SERVICES.expect("Services hasn't been initialised.") }
}

pub(crate) fn services_init(script_manager: IScriptMan) {
    let services = Services {
        act_react: ActReactService {
            service: get_service(&script_manager),
        },
        debug: DebugService {
            service: get_service(&script_manager),
        },
        version: VersionService {
            service: get_service(&script_manager),
        },
    };
    unsafe { SERVICES = Some(Box::leak(Box::new(services))) };
}

fn get_service<T: Interface>(script_manager: &IScriptMan) -> T {
    unsafe { script_manager.GetService(&T::IID).cast::<T>().unwrap() }
}

#[interface("F40000F4-7B74-12C3-8348-00AA00A82B51")]
unsafe trait IActReactServiceT1: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn React(
        &self,
        what: c_int,
        stim_intensity: c_int,
        target: c_int,
        agent: c_int,
        parm1: *const sMultiParm,
        parm2: *const sMultiParm,
        parm3: *const sMultiParm,
        parm4: *const sMultiParm,
        parm5: *const sMultiParm,
        parm6: *const sMultiParm,
        parm7: *const sMultiParm,
        parm8: *const sMultiParm,
    ) -> HRESULT;
    fn StimulateLegacy(&self, who: c_int, what: c_int, how_much: c_float) -> HRESULT;
    fn GetReactionNamed(&self, name: *const c_char) -> c_int;
    fn GetReactionName(&self, id: c_int) -> *const c_char;
    fn SubscribeToStimulus(&self, obj: c_int, what: c_int) -> HRESULT;
    fn UnsubscribeToStimulus(&self, obj: c_int, what: c_int) -> HRESULT;
    fn BeginContact(&self, source: c_int, sensor: c_int) -> HRESULT;
    fn EndContact(&self, source: c_int, sensor: c_int) -> HRESULT;
    fn SetSingleSensorContact(&self, source: c_int, sensor: c_int) -> HRESULT;
    fn Stimulate(&self, who: c_int, what: c_int, how_much: c_float, source: c_int) -> HRESULT;
}

#[interface("F40000F4-7B74-12C3-8348-00AA00A82B51")]
unsafe trait IActReactService: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn React(
        &self,
        what: c_int,
        stim_intensity: c_int,
        target: c_int,
        agent: c_int,
        parm1: *const sMultiParm,
        parm2: *const sMultiParm,
        parm3: *const sMultiParm,
        parm4: *const sMultiParm,
        parm5: *const sMultiParm,
        parm6: *const sMultiParm,
        parm7: *const sMultiParm,
        parm8: *const sMultiParm,
    ) -> HRESULT;
    fn Stimulate(&self, who: c_int, what: c_int, how_much: c_float, source: c_int) -> HRESULT;
    fn GetReactionNamed(&self, name: *const c_char) -> c_int;
    fn GetReactionName(&self, id: c_int) -> *const c_char;
    fn SubscribeToStimulus(&self, obj: c_int, what: c_int) -> HRESULT;
    fn UnsubscribeToStimulus(&self, obj: c_int, what: c_int) -> HRESULT;
    fn BeginContact(&self, source: c_int, sensor: c_int) -> HRESULT;
    fn EndContact(&self, source: c_int, sensor: c_int) -> HRESULT;
    fn SetSingleSensorContact(&self, source: c_int, sensor: c_int) -> HRESULT;
}

pub struct ActReactService {
    #[cfg(game = "t1")]
    service: IActReactServiceT1,
    #[cfg(not(game = "t1"))]
    service: IActReactService,
}

impl ActReactService {
    pub fn react(
        &self,
        what: i32,
        stim_intensity: i32,
        target: Option<i32>,
        agent: Option<i32>,
        parm1: Option<sMultiParm>,
        parm2: Option<sMultiParm>,
        parm3: Option<sMultiParm>,
        parm4: Option<sMultiParm>,
        parm5: Option<sMultiParm>,
        parm6: Option<sMultiParm>,
        parm7: Option<sMultiParm>,
        parm8: Option<sMultiParm>,
    ) {
        let _ = unsafe {
            self.service.React(
                what,
                stim_intensity,
                target.unwrap_or(0),
                agent.unwrap_or(0),
                match parm1 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm2 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm3 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm4 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm5 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm6 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm7 {
                    Some(p) => &p,
                    None => null(),
                },
                match parm8 {
                    Some(p) => &p,
                    None => null(),
                },
            )
        };
    }

    pub fn stimulate(&self, who: i32, what: i32, how_much: f32, source: Option<i32>) {
        let _ = unsafe {
            self.service
                .Stimulate(who, what, how_much, source.unwrap_or(0))
        };
    }

    pub fn get_reaction_named(&self, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        unsafe { self.service.GetReactionNamed(name.as_ptr()) }
    }

    pub fn get_reaction_name(&self, id: i32) -> String {
        unsafe {
            let name = self.service.GetReactionName(id);
            CStr::from_ptr(name).to_str().unwrap().to_string()
        }
    }

    pub fn subscribe_to_stimulus(&self, obj: i32, what: i32) {
        let _ = unsafe { self.service.SubscribeToStimulus(obj, what) };
    }

    pub fn unsubscribe_to_stimulus(&self, obj: i32, what: i32) {
        let _ = unsafe { self.service.UnsubscribeToStimulus(obj, what) };
    }

    pub fn begin_contact(&self, source: i32, sensor: i32) {
        let _ = unsafe { self.service.BeginContact(source, sensor) };
    }

    pub fn end_contact(&self, source: i32, sensor: i32) {
        let _ = unsafe { self.service.EndContact(source, sensor) };
    }

    pub fn set_single_sensor_contact(&self, source: i32, sensor: i32) {
        let _ = unsafe { self.service.SetSingleSensorContact(source, sensor) };
    }
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

#[interface("2A000228-7CA8-13F7-8348-00AA00A82B51")]
unsafe trait IVersionService: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn GetAppName(&self, title_only: BOOL, app_name: &mut *mut c_char);
    fn GetVersion(&self, major: &mut c_int, minor: &mut c_int);
    fn IsEditor(&self) -> c_int;
    fn GetGame(&self, game: &mut *mut c_char);
    fn GetGamsys(&self, gamsys: &mut *mut c_char);
    fn GetMap(&self, map: &mut *mut c_char);
    fn GetCurrentFM(&self, current_fm: &mut *mut c_char) -> HRESULT;
    fn GetCurrentFMPath(&self, current_fm_path: &mut *mut c_char) -> HRESULT;
    fn FMizeRelativePath(&self, in_path: *const c_char, out_path: &mut *mut c_char);
    fn FMizePath(&self, in_path: *const c_char, out_path: &mut *mut c_char);
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

    pub fn get_game(&self) -> String {
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.GetGame(&mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }

    pub fn get_gamsys(&self) -> String {
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.GetGamsys(&mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }

    pub fn get_map(&self) -> String {
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.GetMap(&mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }

    pub fn get_current_fm(&self) -> Option<String> {
        let mut ptr = CString::from(c"").into_raw();
        let result = unsafe { self.service.GetCurrentFM(&mut ptr) };
        let fm = unsafe { CString::from_raw(ptr).to_str().unwrap().to_string() };

        match HRESULT::is_ok(result) {
            true => Some(fm),
            false => None,
        }
    }

    pub fn get_current_fm_path(&self) -> Option<String> {
        let mut ptr = CString::from(c"").into_raw();
        let result = unsafe { self.service.GetCurrentFMPath(&mut ptr) };
        let fm_path = unsafe { CString::from_raw(ptr).to_str().unwrap().to_string() };

        match HRESULT::is_ok(result) {
            true => Some(fm_path),
            false => None,
        }
    }

    pub fn fmize_relative_path(&self, path: &str) -> String {
        let path = CString::from_str(path).unwrap();
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.FMizeRelativePath(path.as_ptr(), &mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }

    pub fn fmize_path(&self, path: &str) -> String {
        let path = CString::from_str(path).unwrap();
        let mut ptr = CString::from(c"").into_raw();
        unsafe {
            self.service.FMizePath(path.as_ptr(), &mut ptr);
            CString::from_raw(ptr).to_str().unwrap().to_string()
        }
    }
}
