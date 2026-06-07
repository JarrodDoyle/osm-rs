/// Standard COM GUID layout (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}

/// The fixed `data4` suffix shared by all LG GUIDs.
const LG_GUID_DATA4: [u8; 8] = [0x83, 0x48, 0x00, 0xAA, 0x00, 0xA8, 0x2B, 0x51];

/// Build a Dark Engine LG-style GUID from a service/interface ID.
///
/// Matches the `DEFINE_LG_GUID(name, id)` macro from the LGS SDK:
/// ```text
/// {((id&0xFF)<<24)|((id&0xFF00)<<16)|(id&0xFFFF),
///  0x7A80+id, 0x11CF+id, {0x83,0x48,0x00,0xAA,0x00,0xA8,0x2B,0x51}}
/// ```
pub const fn lg_guid(id: u32) -> Guid {
    Guid {
        data1: ((id & 0xFF) << 24) | ((id & 0xFF00) << 16) | (id & 0xFFFF),
        data2: (0x7A80 + id) as u16,
        data3: (0x11CF + id) as u16,
        data4: LG_GUID_DATA4,
    }
}

/// Extract the LG service ID from a binary GUID, if it matches the LG pattern.
///
/// Returns `None` if `data4` doesn't match the LG suffix, or if `data2` and
/// `data3` are inconsistent (i.e. don't encode the same ID).
pub const fn lg_id_from_guid(guid: &Guid) -> Option<u16> {
    // Check data4 matches the LG pattern
    let d = &guid.data4;
    if d[0] != 0x83
        || d[1] != 0x48
        || d[2] != 0x00
        || d[3] != 0xAA
        || d[4] != 0x00
        || d[5] != 0xA8
        || d[6] != 0x2B
        || d[7] != 0x51
    {
        return None;
    }
    // Extract ID from data2 and cross-check with data3
    let id_from_d2 = guid.data2.wrapping_sub(0x7A80);
    let id_from_d3 = guid.data3.wrapping_sub(0x11CF);
    if id_from_d2 != id_from_d3 {
        return None;
    }
    Some(id_from_d2)
}

/// All script services exposed by the Dark Engine's IScriptMan::GetService.
macro_rules! script_services {
    ($($name:ident = $id:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u16)]
        pub enum ScriptService {
            $($name = $id),*
        }

        impl ScriptService {
            pub const ALL: &[ScriptService] = &[$(Self::$name),*];

            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $(stringify!($name) => Some(Self::$name),)*
                    _ => None,
                }
            }
        }
    }
}

script_services! {
    Null          = 0x0F2,
    Sound         = 0x0F1,
    ActReact      = 0x0F4,
    Locked        = 0x0FB,
    Puppet        = 0x0FD,
    Damage        = 0x0FE,
    Door          = 0x0F6,
    AI            = 0x0E5,
    Debug         = 0x0D7,
    Object        = 0x0DF,
    Property      = 0x0DA,
    Link          = 0x0EE,
    LinkTools     = 0x0EF,
    Key           = 0x10D,
    Weapon        = 0x10E,
    PickLock      = 0x111,
    Bow           = 0x115,
    Camera        = 0x140,
    Physics       = 0x141,
    DrkInv        = 0x150,
    Inventory     = 0x15E,
    Quest         = 0x152,
    DrkPowerups   = 0x153,
    PlayerLimbs   = 0x15D,
    AnimTexture   = 0x16A,
    Light         = 0x16C,
    Container     = 0x17D,
    DarkUI        = 0x19F,
    Data          = 0x1A0,
    DarkGame      = 0x1B4,
    ShockPsi      = 0x1D7,
    ShockObj      = 0x1D9,
    PGroup        = 0x1F8,
    ShockGame     = 0x108,
    ShockWeapon   = 0x213,
    ShockAI       = 0x21B,
    Networking    = 0x225,
    CD            = 0x226,
    Version       = 0x228,
    Engine        = 0x229,
    ShockOverlay  = 0x22A,
    DarkOverlay   = 0x22B,
}

impl ScriptService {
    /// Returns the short service ID (e.g. `0x17D` for Container).
    pub const fn id(self) -> u16 {
        self as u16
    }

    /// Returns the LG GUID for this service.
    pub const fn guid(self) -> Guid {
        lg_guid(self as u32)
    }

    /// Returns the GUID as a hyphenated uppercase string,
    /// e.g. `"7D00017D-7BFD-134C-8348-00AA00A82B51"`.
    pub fn guid_string(self) -> String {
        self.guid().to_string()
    }
}
