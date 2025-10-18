mod services;

use std::{
    ffi::{CString, c_char, c_int, c_uchar, c_uint, c_ulong},
    os::raw::c_void,
    ptr::{null, null_mut},
    str::FromStr,
};

pub use crate::services::*;
pub use windows::{Win32::System::Com::IMalloc, core::*};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sDatapath {
    pub num: c_uchar,
    pub last: c_uchar,
    pub nocurrent: BOOL,
    pub datapath: [*mut c_char; 16usize],
    pub findflags: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sLink {
    pub source: c_int,
    pub dest: c_int,
    pub flavor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sMultiParm {
    pub val: c_int, // Union
    pub t: c_int,   // enum
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sScrClassDesc {
    pub mod_: *const c_char,
    pub name: *const c_char,
    pub base: *const c_char,
    pub factory: unsafe extern "C" fn(name: *const c_char, obj_id: c_int) -> *mut IScript,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sScrDatumTag {
    pub objId: c_int,
    pub _class: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sScrTraceHashKey {
    pub combo: [c_uchar; 40usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sScrTrace {
    pub hostobj: c_int,
    pub action: c_uint,
    pub line: c_int,
    pub hashkey: sScrTraceHashKey,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sPersistentVtbl {
    pub Destruct: Option<unsafe extern "C" fn(arg1: *mut sPersistentVtbl)>,
    pub Persistence: Option<unsafe extern "C" fn(arg1: *mut sPersistentVtbl) -> BOOL>,
    pub GetName:
        Option<unsafe extern "C" fn(arg1: *mut sPersistentVtbl) -> *const ::std::os::raw::c_char>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sScrMsg {
    pub lpVtbl: *mut IUnknown_Vtbl,
    pub count: c_uint,
    pub lpPersistentVtbl: *mut sPersistentVtbl,
    pub from: c_int,
    pub to: c_int,
    pub message: *const c_char,
    pub time: c_ulong,
    pub flags: c_int,
    pub data: sMultiParm,
    pub data2: sMultiParm,
    pub data3: sMultiParm,
}

#[interface("D00000D0-7B50-129F-8348-00AA00A82B51")]
pub unsafe trait IScript: IUnknown {
    fn GetClassName(&self) -> *const c_char;
    fn ReceiveMessage(&self, msg: &mut sScrMsg, parms: &mut sMultiParm, action: i32) -> HRESULT;
}

#[interface("CF0000CF-7B4F-129E-8348-00AA00A82B51")]
pub unsafe trait IScriptMan: IUnknown {
    fn GameInit(&self) -> HRESULT;
    fn GameEnd(&self) -> HRESULT;
    fn BeginScripts(&self) -> HRESULT;
    fn EndScripts(&self) -> HRESULT;
    fn SetModuleDatapath(&self, datapath: *const sDatapath) -> HRESULT;
    fn AddModule(&self, name: *const c_char) -> HRESULT;
    fn RemoveModule(&self, name: *const c_char) -> HRESULT;
    fn ClearModules(&self) -> HRESULT;
    fn ExposeService(&self, service: IUnknown, guid: *const GUID) -> HRESULT;
    fn GetService(&self, guid: &GUID) -> IUnknown;
    fn GetFirstClass(&self, class: *mut c_uint) -> *const sScrClassDesc;
    fn GetNextClass(&self, class: *mut c_uint) -> *const sScrClassDesc;
    fn EndClassIter(&self, iter: *mut c_uint);
    fn GetClass(&self, name: *const c_char) -> *const sScrClassDesc;
    fn SetObjScripts(&self, obj_id: c_int, names: *mut *const c_char, len: c_uint) -> HRESULT;
    fn ForgetObj(&self, obj_id: c_int) -> HRESULT;
    fn ForgetAllObjs(&self) -> HRESULT;
    fn WantsMessage(&self, obj_id: c_int, msg_name: *const c_char) -> BOOL;
    fn SendMessage(&self, msg: *mut sScrMsg, parms: *mut sMultiParm) -> HRESULT;
    fn KilTimedMessage(&self, msg_id: c_uint);
    fn PumpMessages(&self) -> c_int;
    fn PostMessage(&self, msg: *mut sScrMsg);
    fn SetTimedMessage(&self, msg: *mut sScrMsg, time: c_ulong, kind: c_int) -> c_uint;
    fn SendMessage2(
        &self,
        from: c_int,
        to: c_int,
        msg_name: *const c_char,
        parms1: *const sMultiParm,
        parms2: *const sMultiParm,
        parms3: *const sMultiParm,
    ) -> sMultiParm;
    fn PostMessage2(
        &self,
        from: c_int,
        to: c_int,
        msg_name: *const c_char,
        parms1: *const sMultiParm,
        parms2: *const sMultiParm,
        parms3: *const sMultiParm,
        flags: c_ulong,
    );
    fn SetTimedMessage2(
        &self,
        to: c_int,
        msg_name: *const c_char,
        time: c_ulong,
        kind: c_int,
        parms: *const sMultiParm,
    ) -> c_uint;
    fn IsScriptDataSet(&self, tag: *const sScrDatumTag) -> BOOL;
    fn GetScriptData(&self, tag: *const sScrDatumTag, parms: *mut sMultiParm) -> HRESULT;
    fn SetScriptData(&self, tag: *const sScrDatumTag, parms: *const sMultiParm) -> HRESULT;
    fn ClearScriptData(&self, tag: *const sScrDatumTag, parms: *mut sMultiParm) -> HRESULT;
    fn AddTrace(&self, obj_id: c_int, name: *const c_char, unk1: c_int, unk2: c_int) -> HRESULT;
    fn RemoveTrace(&self, obj_id: c_int, name: *const c_char) -> HRESULT;
    fn GetTraceLine(&self, line: c_int) -> BOOL;
    fn SetTraceLine(&self, line: c_int, on: BOOL);
    fn GetTraceLineMask(&self) -> c_int;
    fn SetTraceLineMask(&self, mask: c_int);
    fn GetFirstTrace(&self, iter: *mut c_uint) -> *const sScrTrace;
    fn GetNextTrace(&self, iter: *mut c_uint) -> *const sScrTrace;
    fn EndTraceIter(&self, iter: *mut c_uint);
    fn SaveLoad(&self, func: *mut c_int, ctx: *mut c_void, loading: BOOL) -> HRESULT;
    fn PostLoad(&self);
}

#[interface("D40000D4-7B54-12A3-8348-00AA00A82B51")]
unsafe trait IScriptModule: IUnknown {
    fn GetName(&self) -> *const c_char;
    fn GetFirstClass(&self, iter: &mut c_uint) -> *const sScrClassDesc;
    fn GetNextClass(&self, iter: &mut c_uint) -> *const sScrClassDesc;
    fn EndClassIter(&self, iter: &mut c_uint);
}

#[implement(IScriptModule)]
pub struct ScriptModule {
    name: CString,
    classes: Vec<sScrClassDesc>,
}

impl ScriptModule {
    pub fn new(name: &str) -> Self {
        Self {
            name: CString::new(name).unwrap(),
            classes: vec![],
        }
    }

    pub fn add_script<T>(&mut self)
    where
        T: DarkScript,
        IScript: From<T>,
    {
        self.classes.push(T::get_desc(self.name.to_str().unwrap()));
    }

    /// # Safety
    ///
    /// `out_mod` must be a non-null, valid pointer for writing an interface pointer.
    pub unsafe fn register(self, out_mod: *mut *mut c_void) -> bool {
        let script_module: IScriptModule = self.into();
        let guid = IScriptModule::IID;
        unsafe {
            if !HRESULT::is_ok(script_module.query(&raw const guid, out_mod)) {
                return false;
            }
        }

        true
    }
}

impl IScriptModule_Impl for ScriptModule_Impl {
    unsafe fn GetName(&self) -> *const c_char {
        self.name.as_ptr()
    }

    unsafe fn GetFirstClass(&self, iter: &mut c_uint) -> *const sScrClassDesc {
        *iter = 0;
        if *iter < self.classes.len() as u32 {
            return &self.classes[*iter as usize];
        }

        null()
    }

    unsafe fn GetNextClass(&self, iter: &mut c_uint) -> *const sScrClassDesc {
        *iter += 1;
        if *iter < self.classes.len() as u32 {
            return &self.classes[*iter as usize];
        }

        null()
    }

    unsafe fn EndClassIter(&self, _: &mut c_uint) {}
}

pub trait DarkScript: Sized
where
    IScript: From<Self>,
    Self: Default,
{
    const NAME: &str;

    fn get_desc(mod_name: &str) -> sScrClassDesc {
        let mod_ = CString::from_str(mod_name).unwrap();
        let name = CString::from_str(Self::NAME).unwrap();
        sScrClassDesc {
            mod_: mod_.into_raw(),
            name: name.into_raw(),
            base: null(),
            factory: script_factory::<Self>,
        }
    }
}

extern "C" fn script_factory<T: Default>(_name: *const c_char, _id: c_int) -> *mut IScript
where
    IScript: From<T>,
{
    unsafe {
        let mut ret: *mut c_void = null_mut();
        let script: IScript = T::default().into();
        let guid = IScript::IID;
        if !HRESULT::is_ok(script.query(&raw const guid, &mut ret)) {
            return null_mut();
        }

        ret as *mut IScript
    }
}
