extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{ parse_macro_input, ItemEnum, LitStr };
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CLI_META_DATA: Lazy<Mutex<Vec<(String, String, String)>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// #[cli_command(name = "Test", args = "TestCommand")]
#[proc_macro_attribute]
pub fn cli_command(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut clicommand_name: Option<LitStr> = None;
    let mut command_name: Option<LitStr> = None;
    let mut command_args: Option<LitStr> = None;

    let handler_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("command") {
            clicommand_name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("name") {
            command_name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("args") {
            command_args = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("expected #[cli_command(name = \"Test\", args = \"TestCommand\")]"))
        }
    });

    parse_macro_input!(args with handler_parser);

    if clicommand_name.is_none() && !command_name.is_none() {
        clicommand_name = command_name.clone();
    }

    CLI_META_DATA.lock().unwrap().push((clicommand_name.unwrap().value(), command_name.unwrap().value(), command_args.unwrap().value()));

    let enum_block = parse_macro_input!(input as ItemEnum);
    // Return the original input unchanged
    quote!(#enum_block).into()
}

#[proc_macro_attribute]
pub fn cli_dispatcher(_args: TokenStream, _input: TokenStream) -> TokenStream {
    let meta_data = CLI_META_DATA.lock().unwrap();

    let match_arms = meta_data.iter().map(|(variant_name, command_name, command_args)| {
        let variant_ident = format_ident!("{}", variant_name);
        let name_ident = format_ident!("{}", command_name);
        let args_ty: syn::Type = syn::parse_str(command_args).expect("Failed to parse args type");

        quote! {
            CliCommand::#variant_ident { arg } => Command::#name_ident(::mustash_core::commands::#args_ty(arg)),
        }
    });

    let expanded = quote! {
        impl From<CliCommand> for Command {
            fn from(cli: CliCommand) -> Self {
                match cli {
                    #(#match_arms)*
                    _ => panic!("Unknown CLI command variant"),
                }
            }
        }
    };

    expanded.into()
}

