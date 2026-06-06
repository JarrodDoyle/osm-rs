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
use syn::{
    Field, Fields, Ident, ItemStruct, Meta, parse::Parser, parse_macro_input,
    punctuated::Punctuated,
};

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

#[proc_macro_attribute]
pub fn dark_script(attr: TokenStream, item: TokenStream) -> TokenStream {
    let messages =
        parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);

    let mut registrations = TokenStream2::default();
    for msg in messages {
        let message = msg.to_token_stream().to_string();
        let message_func = Ident::new(
            &format!("on_{}", message.to_snake_case()),
            Span::call_site(),
        );
        registrations.extend(quote! {
            self.handlers.insert(#message.to_string(), Self::#message_func);
        });
    }

    let mut item = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = item.fields {
        fields.named.push(
            Field::parse_named
                .parse2(
                    quote! {
                        handlers: std::collections::HashMap<String, fn(&Self, &Services, &sScrMsg) -> HRESULT>
                    },
                )
                .unwrap(),
        );
    }

    let name = &item.ident;
    let script_name = name.to_string();
    let script_impl_block = format_ident!("{}_Impl", &name);

    quote! {
        #[implement(IScript)]
        #[derive(Default, Debug)]
        #item

        impl #name {
            fn register_handlers(&mut self) {
                #registrations
            }
        }

        impl IScript_Impl for #script_impl_block {
            unsafe fn GetClassName(&self) -> *const std::ffi::c_char {
                std::ffi::CString::from_str(#script_name).unwrap().into_raw()
            }

            unsafe fn ReceiveMessage(&self, msg: &mut sScrMsg, _: &mut sMultiParm, _: i32) -> HRESULT {
                let services = services();

                let message_name = unsafe {
                    std::ffi::CStr::from_ptr(msg.message).to_str().unwrap()
                };
                if self.handlers.contains_key(message_name) {
                    return self.handlers[message_name](self, services, msg);
                }

                HRESULT(1)
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
                let mut script = Self::default();
                script.register_handlers();
                let script_interface: IScript = script.into();
                let guid = IScript::IID;
                let query_result = unsafe { script_interface.query(&raw const guid, &mut ret) };
                if !HRESULT::is_ok(query_result) {
                    return std::ptr::null_mut();
                }
                ret as *mut IScript
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
