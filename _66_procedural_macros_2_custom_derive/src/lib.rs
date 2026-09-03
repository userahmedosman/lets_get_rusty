extern crate proc_macro;
use proc_macro::{TokenStream};
use syn;
#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let trait_impl = quote::quote! {
        impl Log for #name {
            fn info(&self, message: &str) {
                println!("[INFO] {}: {}", stringify!(#name), message);
            }
            fn warn(&self, message: &str) {
                println!("[WARN] {}: {}", stringify!(#name), message);
            }
            fn error(&self, message: &str) {
                println!("[ERROR] {}: {}", stringify!(#name), message);
            }
        }
    };
    trait_impl.into()
}