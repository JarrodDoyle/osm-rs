use std::ffi::{c_char, c_float, c_int, c_uint, c_ulong};

use kc_osm_proc_macros::DarkMessageData;
use windows::core::*;

use crate::{LinkId, ObjectId, cstr_convert, sMultiParm, sPersistentVtbl, sVector};

pub trait DarkMessageData {
    unsafe fn from_base_msg(msg: &sScrMsg) -> &Self;
}

/// Base message data.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sScrMsg {
    pub lpVtbl: *mut IUnknown_Vtbl,
    pub count: c_uint,
    pub lpPersistentVtbl: *mut sPersistentVtbl,
    pub from: ObjectId,
    pub to: ObjectId,
    pub message: *const c_char,
    pub time: c_ulong,
    pub flags: c_int,
    pub data: sMultiParm,
    pub data2: sMultiParm,
    pub data3: sMultiParm,
}

impl DarkMessageData for sScrMsg {
    unsafe fn from_base_msg(msg: &sScrMsg) -> &Self {
        msg
    }
}

/// Timer message data.
///
/// Received for the following messages:
///  - Timer
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sScrTimerMsg {
    pub base: sScrMsg,
    raw_name: *const c_char,
}

impl sScrTimerMsg {
    #[must_use]
    pub fn timer_name(&self) -> String {
        cstr_convert(self.raw_name)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TweqType {
    Scale,
    Rotate,
    Joints,
    Models,
    Delete,
    Emitter,
    Flicker,
    Lock,
    All,
    Null,
}

impl TryFrom<i32> for TweqType {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Scale),
            1 => Ok(Self::Rotate),
            2 => Ok(Self::Joints),
            3 => Ok(Self::Models),
            4 => Ok(Self::Delete),
            5 => Ok(Self::Emitter),
            6 => Ok(Self::Flicker),
            7 => Ok(Self::Lock),
            8 => Ok(Self::All),
            9 => Ok(Self::Null),
            _ => Err(v),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TweqOperation {
    KillAll,
    RemoveTweq,
    HaltTweq,
    StatusQuo,
    SlayAll,
    FrameEvent,
}

impl TryFrom<i32> for TweqOperation {
    type Error = i32;

    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::KillAll),
            1 => Ok(Self::RemoveTweq),
            2 => Ok(Self::HaltTweq),
            3 => Ok(Self::StatusQuo),
            4 => Ok(Self::SlayAll),
            5 => Ok(Self::FrameEvent),
            _ => Err(v),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TweqDirection {
    Forward,
    Reverse,
}

impl TryFrom<i32> for TweqDirection {
    type Error = i32;

    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Forward),
            1 => Ok(Self::Reverse),
            _ => Err(v),
        }
    }
}

/// Tweq message data.
///
/// Received for the following messages:
///  - TweqComplete
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sTweqMsg {
    pub base: sScrMsg,
    type_raw: c_int,
    operation_raw: c_int,
    direction_raw: c_int,
}

impl sTweqMsg {
    pub fn tweq_type(&self) -> std::result::Result<TweqType, i32> {
        TweqType::try_from(self.type_raw)
    }

    pub fn tweq_operation(&self) -> std::result::Result<TweqOperation, i32> {
        TweqOperation::try_from(self.operation_raw)
    }

    pub fn tweq_direction(&self) -> std::result::Result<TweqDirection, i32> {
        TweqDirection::try_from(self.direction_raw)
    }
}

/// Sound done message data.
///
/// Received for the following messages:
///  - SoundDone
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sSoundDoneMsg {
    pub base: sScrMsg,
    pub coordinates: sVector,
    pub target_object: ObjectId,
    name_raw: *const c_char,
}

impl sSoundDoneMsg {
    pub fn name(&self) -> String {
        cstr_convert(self.name_raw)
    }
}

/// Schema done message data.
///
/// Received for the following messages:
///  - SchemaDone
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sSchemaDoneMsg {
    pub base: sScrMsg,
    pub coordinates: sVector,
    pub target_object: ObjectId,
    name_raw: *const c_char,
}

impl sSchemaDoneMsg {
    pub fn name(&self) -> String {
        cstr_convert(self.name_raw)
    }
}

/// Sim message data.
///
/// Received for the following messages:
///  - Sim
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sSimMsg {
    pub base: sScrMsg,
    starting_raw: c_int,
}

impl sSimMsg {
    pub fn starting(&self) -> bool {
        self.starting_raw > 0
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjType {
    Player,
    RemotePlayer,
    Creature,
    Object,
    Null,
}

impl TryFrom<i32> for ObjType {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Player),
            1 => Ok(Self::RemotePlayer),
            2 => Ok(Self::Creature),
            3 => Ok(Self::Object),
            4 => Ok(Self::Null),
            _ => Err(v),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomChange {
    Enter,
    Exit,
    RoomTransit,
}

impl TryFrom<i32> for RoomChange {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Enter),
            1 => Ok(Self::Exit),
            2 => Ok(Self::RoomTransit),
            _ => Err(v),
        }
    }
}

/// Room transition message data.
///
/// Received for the following messages:
///  - ObjRoomTransit
///  - PlayerRoomEnter
///  - PlayerRoomExit
///  - RemotePlayerRoomEnter
///  - RemotePlayerRoomExit
///  - CreatureRoomEnter
///  - CreatureRoomExit
///  - ObjectRoomEnter
///  - ObjectRoomExit
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sRoomMsg {
    pub base: sScrMsg,
    pub from_obj_id: ObjectId,
    pub to_obj_id: ObjectId,
    pub move_obj_id: ObjectId,
    obj_type_raw: c_int,
    transition_type_raw: c_int,
}

impl sRoomMsg {
    pub fn obj_type(&self) -> std::result::Result<ObjType, i32> {
        ObjType::try_from(self.obj_type_raw)
    }

    pub fn transition_type(&self) -> std::result::Result<RoomChange, i32> {
        RoomChange::try_from(self.transition_type_raw)
    }
}

/// Quest message data.
///
/// Received for the following messages:
///  - QuestChange
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sQuestMsg {
    pub base: sScrMsg,
    name_raw: *const c_char,
    pub old_value: c_int,
    pub new_value: c_int,
}

impl sQuestMsg {
    pub fn name(&self) -> String {
        cstr_convert(self.name_raw)
    }
}

/// Moving terrain message data.
///
/// Received for the following messages:
///  - MovingTerrainWaypoint
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sMovingTerrainMsg {
    pub base: sScrMsg,
    pub waypoint_obj_id: ObjectId,
}

/// Waypoint terrain message data.
///
/// Received for the following messages:
///  - WaypointReached
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sWaypointMsg {
    pub base: sScrMsg,
    pub moving_terrain_obj_id: ObjectId,
}

/// Medium transition message data.
///
/// Received for the following messages:
///  - MediumTransition
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sMediumTransMsg {
    pub base: sScrMsg,
    pub from_type: c_int,
    pub to_type: c_int,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrobLocation {
    World,
    Inv,
    Tool,
    None,
}

impl TryFrom<i32> for FrobLocation {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::World),
            1 => Ok(Self::Inv),
            2 => Ok(Self::Tool),
            3 => Ok(Self::None),
            _ => Err(v),
        }
    }
}

/// Frob message data.
///
/// Received for the following messages:
///  - FrobToolBegin
///  - FrobToolEnd
///  - FrobWorldBegin
///  - FrobWorldEnd
///  - FrobInvBegin
///  - FrobInvEnd
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sFrobMsg {
    pub base: sScrMsg,
    pub src_obj_id: ObjectId,
    pub dest_obj_id: ObjectId,
    pub frobber_obj_id: ObjectId,
    src_location_raw: c_int,
    dest_location_raw: c_int,
    pub sec: c_float,
    abort_raw: c_int,
}

impl sFrobMsg {
    pub fn source_location(&self) -> std::result::Result<FrobLocation, i32> {
        FrobLocation::try_from(self.src_location_raw)
    }

    pub fn destination_location(&self) -> std::result::Result<FrobLocation, i32> {
        FrobLocation::try_from(self.dest_location_raw)
    }

    pub fn abort(&self) -> bool {
        self.abort_raw > 0
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorAction {
    Open = 0,
    Close = 1,
    Opening = 2,
    Closing = 3,
    Halt = 4,
}

impl TryFrom<i32> for DoorAction {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, i32> {
        match v {
            0 => Ok(Self::Open),
            1 => Ok(Self::Close),
            2 => Ok(Self::Opening),
            3 => Ok(Self::Closing),
            4 => Ok(Self::Halt),
            _ => Err(v),
        }
    }
}

/// Door message data.
///
/// Received for the following messages:
///  - DoorOpen
///  - DoorClose
///  - DoorOpening
///  - DoorClosing
///  - DoorHalt
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sDoorMsg {
    pub base: sScrMsg,
    action_raw: c_int,
    prev_action_raw: c_int,
    proxy_raw: c_int,
}

impl sDoorMsg {
    pub fn action(&self) -> std::result::Result<DoorAction, i32> {
        DoorAction::try_from(self.action_raw)
    }

    pub fn previous_action(&self) -> std::result::Result<DoorAction, i32> {
        DoorAction::try_from(self.prev_action_raw)
    }

    pub fn is_proxy(&self) -> bool {
        self.proxy_raw > 0
    }
}

/// Difficulty message data.
///
/// Received for the following messages:
///  - Difficulty
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sDiffScrMsg {
    pub base: sScrMsg,
    pub difficulty: c_int,
}

/// Damage message data.
///
/// Received for the following messages:
///  - Damage
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sDamageScrMsg {
    pub base: sScrMsg,
    pub kind: c_int,
    pub damage: c_int,
    pub culprit_obj_id: ObjectId,
}

/// Slay message data.
///
/// Received for the following messages:
///  - Slain
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sSlayMsg {
    pub base: sScrMsg,
    pub culprit_obj_id: ObjectId,
    pub kind: c_int,
}

/// Container message data.
///
/// Received for the following messages:
///  - Container
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sContainerScrMsg {
    pub base: sScrMsg,
    pub containee_obj_id: ObjectId,
}

/// Contained message data.
///
/// Received for the following messages:
///  - Contained
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sContainedScrMsg {
    pub base: sScrMsg,
    pub container_obj_id: ObjectId,
}

/// Combine message data.
///
/// Received for the following messages:
///  - Combine
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sCombineScrMsg {
    pub base: sScrMsg,
    pub combiner_obj_id: ObjectId,
}

/// Contain message data.
///
/// Received for the following messages:
///  - ContainSimActivate
///  - ContainAdd
///  - ContainRemove
///  - ContainCombine
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sContainMsg {
    pub base: sScrMsg,
    pub container_obj_id: ObjectId,
    pub containee_obj_id: ObjectId,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyAction {
    MotionStart = 0,
    MotionEnd = 1,
    MotionFlagReached = 2,
}

impl TryFrom<i32> for BodyAction {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, i32> {
        match v {
            0 => Ok(Self::MotionStart),
            1 => Ok(Self::MotionEnd),
            2 => Ok(Self::MotionFlagReached),
            _ => Err(v),
        }
    }
}

/// Body message data.
///
/// Received for the following messages:
///  - MotionStart
///  - MotionEnd
///  - MotionFlagReached
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sBodyMsg {
    pub base: sScrMsg,
    action_type_raw: c_int,
    motion_name_raw: *const c_char,
    pub flag_value: c_int,
}

impl sBodyMsg {
    pub fn action(&self) -> std::result::Result<BodyAction, i32> {
        BodyAction::try_from(self.action_type_raw)
    }

    pub fn motion_name(&self) -> String {
        cstr_convert(self.motion_name_raw)
    }
}

/// Attack message data.
///
/// Received for the following messages:
///  - StartWindup
///  - StartAttack
///  - EndAttack
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAttackMsg {
    pub base: sScrMsg,
    pub weapon_obj_id: ObjectId,
}

/// AI signal message data.
///
/// Received for the following messages:
///  - SignalAI
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAISignalMsg {
    pub base: sScrMsg,
    signal_raw: *const c_char,
}

impl sAISignalMsg {
    pub fn signal(&self) -> String {
        cstr_convert(self.signal_raw)
    }
}

/// AI patrol message data.
///
/// Received for the following messages:
///  - PatrolPoint
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAIPatrolPointMsg {
    pub base: sScrMsg,
    pub patrol_obj_id: ObjectId,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIScriptAlertLevel {
    NoAlert,
    LowAlert,
    ModerateAlert,
    HighAlert,
}

impl TryFrom<i32> for AIScriptAlertLevel {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::NoAlert),
            1 => Ok(Self::LowAlert),
            2 => Ok(Self::ModerateAlert),
            3 => Ok(Self::HighAlert),
            _ => Err(v),
        }
    }
}

/// AI alertness message data.
///
/// Received for the following messages:
///  - Alertness
///  - HighAlert
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAIAlertnessMsg {
    pub base: sScrMsg,
    level_raw: c_int,
    previous_level_raw: c_int,
}

impl sAIAlertnessMsg {
    pub fn level(&self) -> std::result::Result<AIScriptAlertLevel, i32> {
        AIScriptAlertLevel::try_from(self.level_raw)
    }

    pub fn previous_level(&self) -> std::result::Result<AIScriptAlertLevel, i32> {
        AIScriptAlertLevel::try_from(self.previous_level_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIMode {
    Asleep,
    SuperEfficient,
    Efficient,
    Normal,
    Combat,
    Dead,
}

impl TryFrom<i32> for AIMode {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Asleep),
            1 => Ok(Self::SuperEfficient),
            2 => Ok(Self::Efficient),
            3 => Ok(Self::Normal),
            4 => Ok(Self::Combat),
            5 => Ok(Self::Dead),
            _ => Err(v),
        }
    }
}

/// AI mode message data.
///
/// Received for the following messages:
///  - AIModeChange
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAIModeChangeMsg {
    pub base: sScrMsg,
    mode_raw: c_int,
    previous_mode_raw: c_int,
}

impl sAIModeChangeMsg {
    pub fn mode(&self) -> std::result::Result<AIMode, i32> {
        AIMode::try_from(self.mode_raw)
    }

    pub fn previous_mode(&self) -> std::result::Result<AIMode, i32> {
        AIMode::try_from(self.previous_mode_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIAction {
    NoAction,
    Goto,
    Frob,
    Maneuver,
}

impl TryFrom<i32> for AIAction {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::NoAction),
            1 => Ok(Self::Goto),
            2 => Ok(Self::Frob),
            3 => Ok(Self::Maneuver),
            _ => Err(v),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIActionResult {
    Done,
    Failed,
    NotAttempted,
}

impl TryFrom<i32> for AIActionResult {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Done),
            1 => Ok(Self::Failed),
            2 => Ok(Self::NotAttempted),
            _ => Err(v),
        }
    }
}

/// AI object action message data.
///
/// Received for the following messages:
///  - ObjActResult
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sAIObjActResultMsg {
    pub base: sScrMsg,
    action_raw: c_int,
    result_raw: c_int,
    pub data: sMultiParm,
    pub target_obj_id: ObjectId,
}

impl sAIObjActResultMsg {
    pub fn action(&self) -> std::result::Result<AIAction, i32> {
        AIAction::try_from(self.action_raw)
    }

    pub fn result(&self) -> std::result::Result<AIActionResult, i32> {
        AIActionResult::try_from(self.result_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysCollisionType {
    None,
    Terrain,
    Object,
}

impl TryFrom<i32> for PhysCollisionType {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Terrain),
            2 => Ok(Self::Object),
            _ => Err(v),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysContactType {
    None,
    Face,
    Edge,
    Vertex,
    Sphere,
    SphereHat,
    OBB,
}

impl TryFrom<i32> for PhysContactType {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Face),
            2 => Ok(Self::Edge),
            3 => Ok(Self::Vertex),
            4 => Ok(Self::Sphere),
            5 => Ok(Self::SphereHat),
            6 => Ok(Self::OBB),
            _ => Err(v),
        }
    }
}

/// Physics message data.
///
/// Received for the following messages:
///  - PhysFellAsleep
///  - PhysWokeUp
///  - PhysMadePhysical
///  - PhysMadeNonPhysical
///  - PhysCollision
///  - PhysContactCreate
///  - PhysContactDestroy
///  - PhysEnter
///  - PhysExit
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sPhysMsg {
    pub base: sScrMsg,
    pub sub_model: c_int,
    collision_type_raw: c_int,
    pub collision_obj_id: ObjectId,
    pub collision_sub_model: c_int,
    pub collision_momentum: c_float,
    pub collision_normal: sVector,
    pub collision_point: sVector,
    contact_type_raw: c_int,
    pub contact_obj_id: ObjectId,
    pub contact_sub_model: c_int,
    pub trans_obj_id: ObjectId,
    pub trans_sub_model: c_int,
}

impl sPhysMsg {
    pub fn collision_type(&self) -> std::result::Result<PhysCollisionType, i32> {
        PhysCollisionType::try_from(self.collision_type_raw)
    }

    pub fn contact_type(&self) -> std::result::Result<PhysContactType, i32> {
        PhysContactType::try_from(self.contact_type_raw)
    }
}

/// Stim message data.
///
/// Received for any message ending in "Stimulus".
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sStimMsg {
    pub base: sScrMsg,
    pub stimulus_obj_id: ObjectId,
    pub intensity: c_float,
    pub sensor_link_id: LinkId,
    pub source_link_id: LinkId,
}

/// Report message data.
///
/// Received for the following messages:
///  - ReportMessage
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sReportMsg {
    pub base: sScrMsg,
    pub warn_level: c_int,
    pub flags: c_int,
    pub types: c_int,
    text_buffer_raw: *const c_char,
}

impl sReportMsg {
    pub fn text_buffer(&self) -> String {
        cstr_convert(self.text_buffer_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyListenMsg {
    Modify,
    Set,
    Unset,
    Load,
    RebuildConcrete,
    RebuildConcreteRelevant,
    RequestFromHost,
}

impl TryFrom<i32> for PropertyListenMsg {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Modify),
            1 => Ok(Self::Set),
            2 => Ok(Self::Unset),
            3 => Ok(Self::Load),
            4 => Ok(Self::RebuildConcrete),
            5 => Ok(Self::RebuildConcreteRelevant),
            6 => Ok(Self::RequestFromHost),
            _ => Err(v),
        }
    }
}

/// Prop notification message data.
///
/// Received for the following messages:
///  - PropNotify
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sPropNotifyMsg {
    pub base: sScrMsg,
    notify_type_raw: c_int,
    prop_name_raw: *const c_char,
    pub obj_id: ObjectId,
    pub donor_obj_id: ObjectId,
}

impl sPropNotifyMsg {
    pub fn notify_type(&self) -> std::result::Result<PropertyListenMsg, i32> {
        PropertyListenMsg::try_from(self.notify_type_raw)
    }

    pub fn prop_name(&self) -> String {
        cstr_convert(self.prop_name_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjNotifyMsg {
    Create,
    Delete,
    LoadObj,
    BeginCreate,
    Database,
    Reset,
    Load,
    Save,
    Default,
    PostLoad,
}

impl TryFrom<i32> for ObjNotifyMsg {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Create),
            1 => Ok(Self::Delete),
            2 => Ok(Self::LoadObj),
            3 => Ok(Self::BeginCreate),
            4 => Ok(Self::Database),
            5 => Ok(Self::Reset),
            6 => Ok(Self::Load),
            7 => Ok(Self::Save),
            8 => Ok(Self::Default),
            9 => Ok(Self::PostLoad),
            _ => Err(v),
        }
    }
}

/// Object notification message data.
///
/// Received for the following messages:
///  - ObjNotify
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sObjNotifyMsg {
    pub base: sScrMsg,
    notify_type_raw: c_int,
    pub obj_id: ObjectId,
}

impl sObjNotifyMsg {
    pub fn notify_type(&self) -> std::result::Result<ObjNotifyMsg, i32> {
        ObjNotifyMsg::try_from(self.notify_type_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationListenMsg {
    Modify,
    Birth,
    Death,
    PostMortem,
}

impl TryFrom<i32> for RelationListenMsg {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Modify),
            1 => Ok(Self::Birth),
            2 => Ok(Self::Death),
            3 => Ok(Self::PostMortem),
            _ => Err(v),
        }
    }
}

/// Link notification message data.
///
/// Received for the following messages:
///  - LinkNotify
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sLinkNotifyMsg {
    pub base: sScrMsg,
    notify_type_raw: c_int,
    pub link_id: LinkId,
    pub source_obj_id: ObjectId,
    pub destination_obj_id: ObjectId,
}

impl sLinkNotifyMsg {
    pub fn notify_type(&self) -> std::result::Result<RelationListenMsg, i32> {
        RelationListenMsg::try_from(self.notify_type_raw)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyMsgKind {
    Added,
    Removed,
}

impl TryFrom<i32> for HierarchyMsgKind {
    type Error = i32;
    fn try_from(v: i32) -> std::result::Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Added),
            1 => Ok(Self::Removed),
            _ => Err(v),
        }
    }
}

/// Trait notification message data.
///
/// Received for the following messages:
///  - TraitNotify
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sTraitNotifyMsg {
    pub base: sScrMsg,
    notify_type_raw: c_int,
    pub obj_id: ObjectId,
    pub donor_obj_id: ObjectId,
}

impl sTraitNotifyMsg {
    pub fn notify_type(&self) -> std::result::Result<HierarchyMsgKind, i32> {
        HierarchyMsgKind::try_from(self.notify_type_raw)
    }
}

/// Dark game mode message data.
///
/// This message is Thief 1/2 only.
///
/// Received for the following messages:
///  - DarkGameModeChange
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sDarkGameModeScrMsg {
    pub base: sScrMsg,
    resuming_raw: c_int,
    suspending_raw: c_int,
}

impl sDarkGameModeScrMsg {
    pub fn resuming(&self) -> bool {
        self.resuming_raw > 0
    }

    pub fn suspending(&self) -> bool {
        self.suspending_raw > 0
    }
}

/// Pick state message data.
///
/// This message is Thief 1/2 only.
///
/// Received for the following messages:
///  - PickStateChange
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sPickStateScrMsg {
    pub base: sScrMsg,
    pub previous_state: c_int,
    pub state: c_int,
}

/// Confirmation message data.
///
/// This message is System Shock 2 only.
///
/// Received for the following messages:
///  - YorNDone
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sYorNMsg {
    pub base: sScrMsg,
    yes_raw: c_int,
}

impl sYorNMsg {
    pub fn yes(&self) -> bool {
        self.yes_raw > 0
    }
}

/// Keypad message data.
///
/// This message is System Shock 2 only.
///
/// Received for the following messages:
///  - KeypadDOne
#[repr(C)]
#[derive(Copy, Clone, Debug, DarkMessageData)]
pub struct sKeypadMsg {
    pub base: sScrMsg,
    pub code: c_int,
}
