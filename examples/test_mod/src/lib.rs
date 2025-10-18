use std::{
    ffi::{CStr, CString, c_char},
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
        let services = services();

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
        let services = services();

        let message_name = unsafe { CStr::from_ptr(msg.message).to_str().unwrap() };
        if message_name == "TurnOn" {
            services.debug.print("Received TurnOn!");
            services.debug.command("run ./cmds/TogglePhys.cmd");
        }

        HRESULT(1)
    }
}

#[unsafe(no_mangle)]
pub extern "Rust" fn register_scripts(module: &mut ScriptModule) {
    module.add_script::<TestScript>();
    module.add_script::<AnotherTestScript>();
}
