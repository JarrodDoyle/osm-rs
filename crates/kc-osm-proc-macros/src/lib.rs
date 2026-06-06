use dark_service_types::ScriptService;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;
use quote::{ToTokens, format_ident, quote};
use syn::Data;
use syn::DeriveInput;
use syn::spanned::Spanned;
use syn::{Ident, ItemStruct, Meta, parse_macro_input, punctuated::Punctuated};

/// Defines a Dark Engine scripting service COM interface.
///
/// Takes a [`ScriptService`] variant name and computes the COM GUID, then
/// delegates to [`windows_core::interface`].
///
/// # Example
/// ```ignore
/// #[dark_service(Container)]
/// unsafe trait IContainerService: IUnknown { ... }
/// ```
///
/// Expands to:
/// ```ignore
/// #[windows_core::interface("7D00017D-7BFD-134C-8348-00AA00A82B51")]
/// unsafe trait IContainerService: IUnknown { ... }
/// ```
#[proc_macro_attribute]
pub fn dark_service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as Ident);
    let guid = match ScriptService::from_name(&ident.to_string()) {
        Some(svc) => svc.guid_string(),
        None => {
            return syn::Error::new_spanned(&ident, format!("Unknown `ScriptService`: `{ident}`"))
                .to_compile_error()
                .into();
        }
    };

    let item: TokenStream2 = item.into();
    quote! {
        #[windows_core::interface(#guid)]
        #item
    }
    .into()
}

/// Returns the message type identifier for a given Dark Engine message name.
fn msg_type_for(message: &str) -> Ident {
    if message.ends_with("Stimulus") {
        return Ident::new("sStimMsg", Span::call_site());
    }

    match message {
        "Timer" => Ident::new("sScrTimerMsg", Span::call_site()),
        "TweqComplete" => Ident::new("sTweqMsg", Span::call_site()),
        "SoundDone" => Ident::new("sSoundDoneMsg", Span::call_site()),
        "SchemaDone" => Ident::new("sSchemaDoneMsg", Span::call_site()),
        "Sim" => Ident::new("sSimMsg", Span::call_site()),
        "ObjRoomTransit"
        | "PlayerRoomEnter"
        | "PlayerRoomExit"
        | "RemotePlayerRoomEnter"
        | "RemotePlayerRoomExit"
        | "CreatureRoomEnter"
        | "CreatureRoomExit"
        | "ObjectRoomEnter"
        | "ObjectRoomExit" => Ident::new("sRoomMsg", Span::call_site()),
        "QuestChange" => Ident::new("sQuestMsg", Span::call_site()),
        "MovingTerrainWaypoint" => Ident::new("sMovingTerrainMsg", Span::call_site()),
        "WaypointReached" => Ident::new("sWaypointMsg", Span::call_site()),
        "MediumTransition" => Ident::new("sMediumTransMsg", Span::call_site()),
        "FrobToolBegin" | "FrobToolEnd" | "FrobWorldBegin" | "FrobWorldEnd" | "FrobInvBegin"
        | "FrobInvEnd" => Ident::new("sFrobMsg", Span::call_site()),
        "DoorOpen" | "DoorClose" | "DoorOpening" | "DoorClosing" | "DoorHalt" => {
            Ident::new("sDoorMsg", Span::call_site())
        }
        "Difficulty" => Ident::new("sDiffScrMsg", Span::call_site()),
        "Damage" => Ident::new("sDamageScrMsg", Span::call_site()),
        "Slain" => Ident::new("sSlayMsg", Span::call_site()),
        "Container" => Ident::new("sContainerScrMsg", Span::call_site()),
        "Contained" => Ident::new("sContainedScrMsg", Span::call_site()),
        "Combine" => Ident::new("sCombineScrMsg", Span::call_site()),
        "ContainSimActivate" | "ContainAdd" | "ContainRemove" | "ContainCombine" => {
            Ident::new("sContainMsg", Span::call_site())
        }
        "MotionStart" | "MotionEnd" | "MotionFlagReached" => {
            Ident::new("sBodyMsg", Span::call_site())
        }
        "StartWindup" | "StartAttack" | "EndAttack" => Ident::new("sAttackMsg", Span::call_site()),
        "SignalAI" => Ident::new("sAISignalMsg", Span::call_site()),
        "PatrolPoint" => Ident::new("sAIPatrolPointMsg", Span::call_site()),
        "Alertness" | "HighAlert" => Ident::new("sAIAlertnessMsg", Span::call_site()),
        "AIModeChange" => Ident::new("sAIModeChangeMsg", Span::call_site()),
        "ObjActResult" => Ident::new("sAIObjActResultMsg", Span::call_site()),
        "PhysFellAsleep"
        | "PhysWokeUp"
        | "PhysMadePhysical"
        | "PhysMadeNonPhysical"
        | "PhysCollision"
        | "PhysContactCreate"
        | "PhysContactDestroy"
        | "PhysEnter"
        | "PhysExit" => Ident::new("sPhysMsg", Span::call_site()),
        "ReportMessage" => Ident::new("sReportMsg", Span::call_site()),
        "PropNotify" => Ident::new("sPropNotifyMsg", Span::call_site()),
        "ObjNotify" => Ident::new("sObjNotifyMsg", Span::call_site()),
        "LinkNotify" => Ident::new("sLinkNotifyMsg", Span::call_site()),
        "TraitNotify" => Ident::new("sTraitNotifyMsg", Span::call_site()),
        "DarkGameModeChange" => Ident::new("sDarkGameModeScrMsg", Span::call_site()),
        "PickStateChange" => Ident::new("sPickStateScrMsg", Span::call_site()),
        "YorNDone" => Ident::new("sYorNMsg", Span::call_site()),
        "KeypadDone" => Ident::new("sKeypadMsg", Span::call_site()),
        _ => Ident::new("sScrMsg", Span::call_site()),
    }
}

#[proc_macro_attribute]
pub fn dark_script(attr: TokenStream, item: TokenStream) -> TokenStream {
    let messages =
        parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);

    let mut match_arms = TokenStream2::default();
    for msg in &messages {
        let message = msg.to_token_stream().to_string();
        let message_func = Ident::new(
            &format!("on_{}", message.to_snake_case()),
            Span::call_site(),
        );

        let msg_type = msg_type_for(&message);
        match_arms.extend(quote! {
            #message => self.#message_func(services, #msg_type::from_base_msg(msg)),
        });
    }

    let item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;
    let script_name = name.to_string();
    let script_impl_block = format_ident!("{}_Impl", &name);

    quote! {
        #[implement(IScript)]
        #[derive(Default, Debug)]
        #item

        impl IScript_Impl for #script_impl_block {
            unsafe fn GetClassName(&self) -> *const std::ffi::c_char {
                std::ffi::CString::from_str(#script_name).unwrap().into_raw()
            }

            unsafe fn ReceiveMessage(&self, msg: &mut sScrMsg, _: &mut sMultiParm, _: i32) -> HRESULT {
                let services = services();
                let message_name = unsafe {
                    std::ffi::CStr::from_ptr(msg.message).to_str().unwrap()
                };

                match message_name {
                    #match_arms
                    _ => HRESULT(1),
                }
            }
        }

        impl DarkScript for #name {
            fn get_desc(mod_name: &str) -> sScrClassDesc {
                let mod_ = std::ffi::CString::from_str(mod_name).unwrap();
                let name = std::ffi::CString::from_str(#script_name).unwrap();
                sScrClassDesc {
                    mod_: mod_.into_raw(),
                    name: name.into_raw(),
                    base: std::ptr::null(),
                    factory: Self::factory,
                }
            }

            extern "C" fn factory(
                _name: *const std::ffi::c_char,
                _id: std::ffi::c_int
            ) -> *mut IScript {
                let mut ret: *mut std::ffi::c_void = std::ptr::null_mut();
                let script: IScript = Self::default().into();
                let guid = IScript::IID;
                let query_result = unsafe { script.query(&raw const guid, &mut ret) };
                if !HRESULT::is_ok(query_result) {
                    return std::ptr::null_mut();
                }
                ret as *mut IScript
            }
        }
    }
    .into()
}

#[proc_macro_derive(DarkMessageData)]
pub fn dark_message_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    quote! {
        impl DarkMessageData for #ident {
            /// Cast a base [`sScrMsg`] to [`#ident`].
            ///
            /// # Safety
            /// Caller must ensure the underlying data is actually of type [`#ident`].
            unsafe fn from_base_msg(msg: &sScrMsg) -> &Self {
                unsafe { &*(msg as *const sScrMsg as *const Self) }
            }
        }
    }
    .into()
}

#[proc_macro_derive(EdittableStruct, attributes(field_desc))]
pub fn struct_editor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let ident_string = ident.to_string();
    let input_fields = match &input.data {
        Data::Struct(data) => data.clone().fields,
        _ => {
            return syn::Error::new_spanned(&input, "Builder can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let mut field_descs = TokenStream2::default();
    for field in input_fields {
        let Some(ref field_ident) = field.ident else {
            return syn::Error::new_spanned(&field, "Unnamed field is not supported")
                .to_compile_error()
                .into();
        };

        let field_ident_string = field_ident.to_string();
        let field_ty = &field.ty;
        for attr in &field.attrs {
            if !attr.path().is_ident("field_desc") {
                continue;
            }

            field_descs.extend(quote_spanned! {
                field_ty.span() =>
                kc_osm::dialogs::FieldDesc::new(
                    #field_ident_string,
                    self.#field_ident.into(),
                    core::mem::size_of::<#field_ty>(),
                    core::mem::offset_of!(#ident, #field_ident)),
            });
        }
    }

    quote! {
        impl kc_osm::dialogs::EdittableStruct for #ident {
            fn edit_struct(&mut self) -> bool {
                let ed = kc_osm::dialogs::construct_struct_editor(
                    &kc_osm::dialogs::StructEditorDesc::new("Struct Editor", 0),
                    &kc_osm::dialogs::StructDesc::new(#ident_string, core::mem::size_of::<#ident>() as u32, 0, &[#field_descs]),
                    self as *mut _ as *mut c_void);
                ed.go(true)
            }
        }
    }
    .into()
}
