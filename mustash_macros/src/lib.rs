extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, LitStr, Ident, ImplItem};

use once_cell::sync::Lazy;
use std::sync::Mutex;

static META_DATA: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::new()));

//
// #[command_handler(command = "Command")]
//
#[proc_macro_attribute]
pub fn command_handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut command_name: Option<LitStr> = None;

    let handler_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("command") {
            command_name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("expected #[command_handler(command = \"Command\")]"))
        }
    });

    parse_macro_input!(args with handler_parser);

    let impl_block = parse_macro_input!(input as ItemImpl);
    let handler_ty = impl_block.self_ty.clone();
    let handler_ty_str = quote!(#handler_ty).to_string();

    if let Some(command_lit) = command_name {
        META_DATA.lock().unwrap().push((handler_ty_str, command_lit.value()));
    }

    // Return the original input unchanged
    quote!(#impl_block).into()
}

//
// #[dispatcher] on `impl Dispatcher` block
//
#[proc_macro_attribute]
pub fn dispatcher(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    let meta_data = META_DATA.lock().unwrap();

    let mut arms = Vec::new();

    for (handler_ty_str, command_str) in meta_data.iter() {
        let command_ident = Ident::new(command_str, proc_macro2::Span::call_site());
        let handler_path: proc_macro2::TokenStream = handler_ty_str.parse().unwrap();

        arms.push(quote! {
            Command::#command_ident(cmd) => crate::handlers::#handler_path::handle_command(cmd),
        });
    }

    let dispatch_fn: ImplItem = syn::parse_quote! {
        pub fn dispatch(&mut self, cmd: Command) -> Response {
            match cmd {
                #(#arms)*
                _ => panic!("Unknown command"),
            }
        }
    };

    impl_block.items.push(dispatch_fn);

    quote!(#impl_block).into()
}

