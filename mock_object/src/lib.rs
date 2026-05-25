mod attr_args;
mod mock_object_impl;
mod mock_template;

use attr_args::AttrArgs;
use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn mock_object(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttrArgs);
    let item = parse_macro_input!(item as ItemStruct);
    mock_object_impl::mock_object_impl(attr, item).into()
}
