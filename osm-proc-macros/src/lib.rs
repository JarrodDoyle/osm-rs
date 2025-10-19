use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Field, Fields, Ident, ItemStruct, Meta, parse::Parser, parse_macro_input,
    punctuated::Punctuated,
};

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
