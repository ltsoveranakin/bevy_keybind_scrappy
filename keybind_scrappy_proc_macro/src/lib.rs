mod key_macro;

extern crate proc_macro;

use proc_macro::{TokenStream};
use std::collections::HashMap;
use std::fs::File;
use quote::quote;
use serde_json::{from_reader, Value};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn parse_json_file(input: TokenStream) -> TokenStream{
    let file_path = parse_macro_input!(input as LitStr).value();

    let file = File::open(file_path).expect("Failed to open file");
    let json_data: HashMap<String, String> = from_reader(file).expect("Failed to read JSON");

    let mut actions = Vec::new();
    for (action_name, action_key) in json_data {
        let action_tokens = (quote! {
            define_action!(#action_name, KeyCode::Key#action_key);
        });
        actions.push(action_tokens);
    }

    actions.into()
}