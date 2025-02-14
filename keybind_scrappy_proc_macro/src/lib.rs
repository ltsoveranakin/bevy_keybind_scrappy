extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde_json::from_reader;
use std::collections::HashMap;
use std::fs::File;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn parse_json_file(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr).value();

    let file = File::open(file_path).expect("Failed to open file");
    let json_data: HashMap<String, String> = from_reader(file).expect("Failed to read JSON");

    let mut actions = Vec::new();
    for (action_name, action_key) in json_data {
        let action_name_ident = Ident::new(&action_name, proc_macro::Span::call_site().into());
        let variant_name = if action_key.len() == 1 {
            format!("Key{}", action_key)
        } else {
            action_key
        };

        let action_key_ident = Ident::new(&variant_name, proc_macro::Span::call_site().into());

        let action = quote!(
            define_action!(#action_name_ident, KeyCode::#action_key_ident);
        );
        actions.push(action);
    }

    let expanded = quote! (
        #(#actions)*
    );

    expanded.into()
}
