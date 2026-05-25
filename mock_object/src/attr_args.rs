use crate::mock_template::MockTemplate;
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

pub struct AttrArgs {
    pub mock_templates: Vec<MockTemplate>,
}

impl Parse for AttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let templates = input
            .parse_terminated(MockTemplate::parse, Token![,])
            .expect("There was a problem parsing list of mock templates");

        if !input.is_empty() {
            return Err(input.error("The input is not empty at the end of the attribute"));
        }

        Ok(Self {
            mock_templates: templates.into_iter().collect(),
        })
    }
}
