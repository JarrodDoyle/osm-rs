use std::result::Result;

use kc_osm::{messages::*, *};
use std::str::FromStr;

#[dark_script(BeginScript, TurnOn)]
pub struct TestScript {}

impl TestScript {
    pub fn on_begin_script(&self, services: &Services, _: &sScrMsg, _: &mut sMultiParm) -> HRESULT {
        let (Some(debug), Some(version)) = (&services.debug, &services.version) else {
            return HRESULT(1);
        };

        let is_editor = version.is_editor();
        let (major, minor) = version.get_version();
        let app_name = version.get_app_name(true);
        debug.print(&format!("is_editor: {is_editor}"));
        debug.print(&format!("app_name: {app_name}"));
        debug.print(&format!("version: {major}.{minor}"));
        debug.print("Wowzers");
        HRESULT(1)
    }

    pub fn on_turn_on(&self, services: &Services, _: &sScrMsg, _: &mut sMultiParm) -> HRESULT {
        let Some(debug) = &services.debug else {
            return HRESULT(1);
        };

        debug.print("Handling TurnOn in TestScript");
        HRESULT(1)
    }
}

#[dark_script(TurnOn, FrobWorldBegin)]
pub struct AnotherTestScript {}

impl AnotherTestScript {
    pub fn on_turn_on(&self, services: &Services, _: &sScrMsg, _: &mut sMultiParm) -> HRESULT {
        let Some(debug) = &services.debug else {
            return HRESULT(1);
        };

        debug.print("Handling TurnOn in AnotherTestScript");
        debug.command("run ./cmds/TogglePhys.cmd");
        HRESULT(1)
    }

    pub fn on_frob_world_begin(
        &self,
        services: &Services,
        _: &sFrobMsg,
        _: &mut sMultiParm,
    ) -> HRESULT {
        let Some(debug) = &services.debug else {
            return HRESULT(1);
        };

        debug.print("Handling FrobWorldBegin in AnotherTestScript");
        HRESULT(1)
    }
}

#[unsafe(no_mangle)]
pub extern "Rust" fn module_init(module: &mut ScriptModule) -> Result<(), String> {
    module.register_script::<TestScript>();
    module.register_script::<AnotherTestScript>();

    Ok(())
}
