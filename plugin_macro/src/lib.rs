use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn plugin_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        #[no_mangle]
        #[allow(improper_ctypes_definitions)]
        pub extern "C" fn create_plugin() -> Box<dyn plugin_core::plugin_api::Plugin> {
            #struct_name::load();
        }
    };

    TokenStream::from(expanded)
}

/// 定义 `#[plugin_exit]` 宏
#[proc_macro_attribute]
pub fn plugin_exit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let expanded = quote! {
        #input

        #[no_mangle]
        #[allow(improper_ctypes_definitions)]
        pub extern "C" fn unload_plugin(plugin: &mut Box<dyn plugin_core::plugin_api::Plugin>) {
            if plugin.is_null() {
                return;
            }
            let plugin = unsafe { Box::from_raw(plugin) };
            plugin.unload();
        }
    };

    TokenStream::from(expanded)
}
