use syn::{
    Expr, Ident, Token, bracketed,
    parse::{Parse, ParseStream},
};

pub struct MockTemplate {
    pub name: Ident,
    pub args: Vec<Expr>,
}

impl Parse for MockTemplate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse().expect("Problem with identifier");
        input
            .parse::<Token![=]>()
            .expect("Problem with parsing equals sign");

        let content;
        bracketed!(content in input);

        let args = content
            .parse_terminated(Expr::parse, Token![,])
            .expect("Problem parsing bracketed list");

        Ok(Self {
            name,
            args: args.into_iter().collect(),
        })
    }
}
