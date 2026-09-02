extern crate proc_macro;
use proc_macro::{TokenStream};

#[proc_macro]
pub fn login_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    for token in input {
        let token_string = token.to_string();
        match token_string.as_str() {
            "[TIME]" => {
                let time = chrono::Local::now();
                output.push_str(&format!(" [{}]", time.format("%Y-%m-%d %H:%M:%S")));
            },
            _ => {
                output.push_str(&format!(" {}", token_string.to_string()));
            }
        }
    }
    TokenStream::from(quote::quote! {
        println!("{}", #output);
    })
}