use std::{
    ffi::{CStr, CString, c_float, c_int},
    os::raw::{c_char, c_void},
    ptr::{null, null_mut},
    str::FromStr,
};

use windows::core::*;

use crate::{IScriptMan, malloc, sMultiParm, sVector};

static mut SERVICES: Option<&Services> = None;

pub struct Services {
    pub act_react: ActReactService,
    pub debug: DebugService,
    pub engine: EngineService,
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
        engine: EngineService {
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
            let value = CStr::from_ptr(name).to_str().unwrap().to_string();
            malloc::free(name as *const c_void);
            value
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

#[interface("2B000229-7CA9-13F8-8348-00AA00A82B51")]
unsafe trait IEngineService: IUnknown {
    fn Init(&self);
    fn End(&self);
    fn ConfigIsDefined(&self, name: *const c_char) -> BOOL;
    fn ConfigGetInt(&self, name: *const c_char, value: *mut c_int) -> BOOL;
    fn ConfigGetFloat(&self, name: *const c_char, value: *mut c_float) -> BOOL;
    fn ConfigGetRaw(&self, name: *const c_char, value: *mut *mut c_char) -> BOOL;
    fn BindingGetFloat(&self, name: *const c_char) -> c_float;
    fn FindFileInPath(
        &self,
        path_config_var: *const c_char,
        filename: *const c_char,
        fullname: *mut *mut c_char,
    ) -> BOOL;
    fn IsRunningDX6(&self) -> BOOL;
    fn GetCanvasSize(&self, width: *mut c_int, height: *mut c_int);
    fn GetAspectRatio(&self) -> c_float;
    fn GetFog(&self, r: *mut c_int, g: *mut c_int, b: *mut c_int, dist: *mut c_float);
    fn SetFog(&self, r: c_int, g: c_int, b: c_int, dist: c_float);
    fn GetFogZone(
        &self,
        zone: c_int,
        r: *mut c_int,
        g: *mut c_int,
        b: *mut c_int,
        dist: *mut c_float,
    );
    fn SetFogZone(&self, zone: c_int, r: c_int, g: c_int, b: c_int, dist: c_float);
    fn GetWeather(
        &self,
        precip_type: *mut c_int,
        precip_freq: *mut c_float,
        precip_speed: *mut c_float,
        vis_dist: *mut c_float,
        rend_radius: *mut c_float,
        alpha: *mut c_float,
        brightness: *mut c_float,
        snow_jitter: *mut c_float,
        rain_len: *mut c_float,
        splash_freq: *mut c_float,
        splash_radius: *mut c_float,
        splash_height: *mut c_float,
        splash_duration: *mut c_float,
        texture: *mut *mut c_char,
        wind: *mut sVector,
    );
    fn SetWeather(
        &self,
        precip_type: c_int,
        precip_freq: c_float,
        precip_speed: c_float,
        vis_dist: c_float,
        rend_radius: c_float,
        alpha: c_float,
        brightness: c_float,
        snow_jitter: c_float,
        rain_len: c_float,
        splash_freq: c_float,
        splash_radius: c_float,
        splash_height: c_float,
        splash_duration: c_float,
        texture: *const c_char,
        wind: *const sVector,
    );
}

pub struct FogSettings {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub distance: f32,
}

pub struct WeatherSettings {
    pub precipitation_type: i32,
    pub precipitation_frequency: f32,
    pub precipitation_speed: f32,
    pub visibility_distance: f32,
    pub render_radius: f32,
    pub alpha: f32,
    pub brightness: f32,
    pub snow_jitter: f32,
    pub rain_length: f32,
    pub splash_frequency: f32,
    pub splash_radius: f32,
    pub splash_height: f32,
    pub splash_duration: f32,
    pub texture: String,
    pub wind: sVector,
}

pub struct EngineService {
    service: IEngineService,
}

impl EngineService {
    pub fn config_is_defined(&self, name: &str) -> bool {
        let name = CString::from_str(name).unwrap();
        unsafe { self.service.ConfigIsDefined(name.as_ptr()).into() }
    }

    pub fn config_get_int(&self, name: &str) -> Option<i32> {
        let name = CString::from_str(name).unwrap();
        let mut value = 0;
        match unsafe { self.service.ConfigGetInt(name.as_ptr(), &mut value).into() } {
            true => Some(value),
            false => None,
        }
    }

    pub fn config_get_float(&self, name: &str) -> Option<f32> {
        let name = CString::from_str(name).unwrap();
        let mut value = 0.0;
        match unsafe {
            self.service
                .ConfigGetFloat(name.as_ptr(), &mut value)
                .into()
        } {
            true => Some(value),
            false => None,
        }
    }

    pub fn config_get_raw(&self, name: &str) -> Option<String> {
        let name = CString::from_str(name).unwrap();
        let mut ptr = null_mut();
        unsafe {
            let val = match self.service.ConfigGetRaw(name.as_ptr(), &mut ptr).into() {
                true => Some(CStr::from_ptr(ptr).to_string_lossy().into_owned()),
                false => None,
            };
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn binding_get_float(&self, name: &str) -> f32 {
        let name = CString::from_str(name).unwrap();
        unsafe { self.service.BindingGetFloat(name.as_ptr()) }
    }

    pub fn find_file_in_path(&self, path_config_var: &str, filename: &str) -> Option<String> {
        let path_config_var = CString::from_str(path_config_var).unwrap();
        let filename = CString::from_str(filename).unwrap();
        let mut ptr = null_mut();
        unsafe {
            let val = match self
                .service
                .FindFileInPath(path_config_var.as_ptr(), filename.as_ptr(), &mut ptr)
                .into()
            {
                true => Some(CStr::from_ptr(ptr).to_string_lossy().into_owned()),
                false => None,
            };
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn is_running_dx6(&self) -> bool {
        unsafe { self.service.IsRunningDX6().into() }
    }

    pub fn get_canvas_size(&self) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        unsafe { self.service.GetCanvasSize(&mut width, &mut height) };
        (width, height)
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        unsafe { self.service.GetAspectRatio() }
    }

    pub fn get_fog(&self) -> FogSettings {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut distance = 0.0;
        unsafe { self.service.GetFog(&mut r, &mut g, &mut b, &mut distance) };
        FogSettings { r, g, b, distance }
    }

    pub fn set_fog(&self, fog: &FogSettings) {
        unsafe { self.service.SetFog(fog.r, fog.g, fog.b, fog.distance) };
    }

    pub fn get_fog_zone(&self, zone: i32) -> FogSettings {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut distance = 0.0;
        unsafe {
            self.service
                .GetFogZone(zone, &mut r, &mut g, &mut b, &mut distance)
        };
        FogSettings { r, g, b, distance }
    }

    pub fn set_fog_zone(&self, zone: i32, fog: &FogSettings) {
        unsafe {
            self.service
                .SetFogZone(zone, fog.r, fog.g, fog.b, fog.distance)
        };
    }

    pub fn get_weather(&self) -> WeatherSettings {
        let mut precipitation_type = 0;
        let mut precipitation_frequency = 0.0;
        let mut precipitation_speed = 0.0;
        let mut visibility_distance = 0.0;
        let mut render_radius = 0.0;
        let mut alpha = 0.0;
        let mut brightness = 0.0;
        let mut snow_jitter = 0.0;
        let mut rain_length = 0.0;
        let mut splash_frequency = 0.0;
        let mut splash_radius = 0.0;
        let mut splash_height = 0.0;
        let mut splash_duration = 0.0;
        let mut texture_ptr = null_mut();
        let wind_ptr = null_mut();
        unsafe {
            self.service.GetWeather(
                &mut precipitation_type,
                &mut precipitation_frequency,
                &mut precipitation_speed,
                &mut visibility_distance,
                &mut render_radius,
                &mut alpha,
                &mut brightness,
                &mut snow_jitter,
                &mut rain_length,
                &mut splash_frequency,
                &mut splash_radius,
                &mut splash_height,
                &mut splash_duration,
                &mut texture_ptr,
                wind_ptr,
            );
            let val = WeatherSettings {
                precipitation_type,
                precipitation_frequency,
                precipitation_speed,
                visibility_distance,
                render_radius,
                alpha,
                brightness,
                snow_jitter,
                rain_length,
                splash_frequency,
                splash_radius,
                splash_height,
                splash_duration,
                texture: CStr::from_ptr(texture_ptr).to_string_lossy().into_owned(),
                wind: *wind_ptr,
            };
            // TODO: wind!?
            malloc::free(texture_ptr as *const c_void);
            val
        }
    }

    pub fn set_weather(&self, weather: &WeatherSettings) {
        unsafe {
            let texture = CString::from_str(&weather.texture).unwrap();
            self.service.SetWeather(
                weather.precipitation_type,
                weather.precipitation_frequency,
                weather.precipitation_speed,
                weather.visibility_distance,
                weather.render_radius,
                weather.alpha,
                weather.brightness,
                weather.snow_jitter,
                weather.rain_length,
                weather.splash_frequency,
                weather.splash_radius,
                weather.splash_height,
                weather.splash_duration,
                texture.as_ptr(),
                &weather.wind,
            );
        }
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
        let mut ptr = null_mut();
        unsafe {
            self.service.GetAppName(title_only.into(), &mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
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
        let mut ptr = null_mut();
        unsafe {
            self.service.GetGame(&mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn get_gamsys(&self) -> String {
        let mut ptr = null_mut();
        unsafe {
            self.service.GetGamsys(&mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn get_map(&self) -> String {
        let mut ptr = null_mut();
        unsafe {
            self.service.GetMap(&mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn get_current_fm(&self) -> Option<String> {
        let mut ptr = null_mut();
        let result = unsafe { self.service.GetCurrentFM(&mut ptr) };
        let fm = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
        unsafe { malloc::free(ptr as *const c_void) };

        match HRESULT::is_ok(result) {
            true => Some(fm),
            false => None,
        }
    }

    pub fn get_current_fm_path(&self) -> Option<String> {
        let mut ptr = null_mut();
        let result = unsafe { self.service.GetCurrentFMPath(&mut ptr) };
        let fm_path = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
        unsafe { malloc::free(ptr as *const c_void) };

        match HRESULT::is_ok(result) {
            true => Some(fm_path),
            false => None,
        }
    }

    pub fn fmize_relative_path(&self, path: &str) -> String {
        let path = CString::from_str(path).unwrap();
        let mut ptr = null_mut();
        unsafe {
            self.service.FMizeRelativePath(path.as_ptr(), &mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
        }
    }

    pub fn fmize_path(&self, path: &str) -> String {
        let path = CString::from_str(path).unwrap();
        let mut ptr = null_mut();
        unsafe {
            self.service.FMizePath(path.as_ptr(), &mut ptr);
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            malloc::free(ptr as *const c_void);
            val
        }
    }
}
