extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::ToTokens;
use syn;

use darling::FromMeta;

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args:TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(err) => return err.write_errors().into(),
    };
    impl_log_call(&attr_args, &mut input).into()
}
fn impl_log_call(attr_args:&MacroArgs, input: &mut syn::ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    input.block.stmts.insert(0, syn::parse_quote! {
        println!("[Info] calling: {}", stringify!(#fn_name));
    });
  input.to_token_stream().into()
}

