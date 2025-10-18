use std::{
    ffi::{CStr, CString, c_char},
    os::raw::c_void,
    str::FromStr,
};

use osm::*;

#[implement(IScript)]
#[derive(Default)]
pub struct TestScript;
impl DarkScript for TestScript {
    const NAME: &str = "TestScript";
}
impl IScript_Impl for TestScript_Impl {
    unsafe fn GetClassName(&self) -> *const c_char {
        CString::from_str(TestScript::NAME).unwrap().into_raw()
    }

    unsafe fn ReceiveMessage(&self, msg: &mut sScrMsg, _: &mut sMultiParm, _: i32) -> HRESULT {
        let services = unsafe { SERVICES.expect("") };

        let message_name = unsafe { CStr::from_ptr(msg.message).to_str().unwrap() };
        if message_name == "BeginScript" {
            let is_editor = services.version.is_editor();
            let (major, minor) = services.version.get_version();
            let app_name = services.version.get_app_name(true);
            let script_name = TestScript::NAME;
            services.debug.print(&format!("is_editor: {is_editor}"));
            services.debug.print(&format!("app_name: {app_name}"));
            services.debug.print(&format!("version: {major}.{minor}"));
            services.debug.print(&format!("script: {script_name}"));
        }

        HRESULT(1)
    }
}

#[implement(IScript)]
#[derive(Default)]
pub struct AnotherTestScript;
impl DarkScript for AnotherTestScript {
    const NAME: &str = "AnotherTestScript";
}
impl IScript_Impl for AnotherTestScript_Impl {
    unsafe fn GetClassName(&self) -> *const c_char {
        CString::from_str(AnotherTestScript::NAME)
            .unwrap()
            .into_raw()
    }

    unsafe fn ReceiveMessage(&self, msg: &mut sScrMsg, _: &mut sMultiParm, _: i32) -> HRESULT {
        let services = unsafe { SERVICES.expect("") };

        let message_name = unsafe { CStr::from_ptr(msg.message).to_str().unwrap() };
        if message_name == "TurnOn" {
            services.debug.print("Received TurnOn!");
            services.debug.command("run ./cmds/TogglePhys.cmd");
        }

        HRESULT(1)
    }
}

#[unsafe(no_mangle)]
unsafe extern "stdcall" fn ScriptModuleInit(
    name: *const c_char,
    script_manager: IScriptMan,
    _: *mut i32,
    _: *mut IMalloc,
    out_mod: *mut *mut c_void,
) -> i32 {
    unsafe {
        SERVICES = Some(Box::leak(Box::new(Services::new(script_manager))));

        let mod_name = CStr::from_ptr(name).to_str().unwrap();
        let test_mod: IScriptModule = ScriptModule {
            name: CStr::from_ptr(name).into(),
            classes: vec![
                TestScript::get_desc(mod_name),
                AnotherTestScript::get_desc(mod_name),
            ],
        }
        .into();
        let guid = IScriptModule::IID;
        if !HRESULT::is_ok(test_mod.query(&raw const guid, out_mod)) {
            return false.into();
        }
    }

    true.into()
}
