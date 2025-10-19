use osm::*;
use std::str::FromStr;

#[dark_script(BeginScript, TurnOn)]
pub struct TestScript {}

impl TestScript {
    pub fn on_begin_script(&self, services: &Services, _msg: &sScrMsg) -> HRESULT {
        let is_editor = services.version.is_editor();
        let (major, minor) = services.version.get_version();
        let app_name = services.version.get_app_name(true);
        services.debug.print(&format!("is_editor: {is_editor}"));
        services.debug.print(&format!("app_name: {app_name}"));
        services.debug.print(&format!("version: {major}.{minor}"));
        services.debug.print("Wowzers");
        HRESULT(1)
    }

    pub fn on_turn_on(&self, services: &Services, _msg: &sScrMsg) -> HRESULT {
        services.debug.print("Handling TurnOn in TestScript");
        HRESULT(1)
    }
}

#[dark_script(TurnOn)]
pub struct AnotherTestScript {}

impl AnotherTestScript {
    pub fn on_turn_on(&self, services: &Services, _msg: &sScrMsg) -> HRESULT {
        services.debug.print("Handling TurnOn in AnotherTestScript");
        services.debug.command("run ./cmds/TogglePhys.cmd");
        HRESULT(1)
    }
}

#[unsafe(no_mangle)]
pub extern "Rust" fn register_scripts(module: &mut ScriptModule) {
    module.add_script::<TestScript>();
    module.add_script::<AnotherTestScript>();
}
