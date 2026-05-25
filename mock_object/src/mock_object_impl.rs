use crate::attr_args::AttrArgs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn mock_object_impl(attr: AttrArgs, item: ItemStruct) -> TokenStream {
    let methods = attr.mock_templates.iter().map(|template| {
        let method_name = &template.name;
        let constructor_args = &template.args;
        quote! {
            pub fn #method_name() -> Self {
                Self::new(#(#constructor_args),*)
            }
        }
    });

    let struct_name = &item.ident;

    quote! {
        #item

        impl #struct_name {
            #(#methods)*
        }
    }
}
