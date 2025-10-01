mod define_coins_with_codes;
mod define_map;
mod model_enum;
mod model_options;
mod model_struct;

use proc_macro::TokenStream;
use syn::{Error, Item, parse_macro_input};

#[proc_macro_attribute]
pub fn model(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item {
        Item::Enum(enum_) => match model_enum::generate(attr.into(), enum_) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.into_compile_error().into(),
        },
        Item::Struct(struct_) => match model_struct::generate(attr.into(), struct_) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.into_compile_error().into(),
        },
        unsupported => Error::new_spanned(unsupported, "不支持的类型")
            .into_compile_error()
            .into(),
    }
}

#[proc_macro]
pub fn define_map(input: TokenStream) -> TokenStream {
    parse_macro_input!(input with define_map::generate).into()
}

#[proc_macro]
pub fn define_coins_with_codes(_input: TokenStream) -> TokenStream {
    match define_coins_with_codes::generate() {
        Ok(ast) => ast.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
