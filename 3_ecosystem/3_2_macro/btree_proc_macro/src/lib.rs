use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, Error, ExprTuple, Token,
};
struct BTreeProcMacro {
    punctuated: Punctuated<ExprTuple, Comma>,
}

impl Parse for BTreeProcMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = input.parse_terminated(ExprTuple::parse, Token![,])?;
        for tup in &content {
            if tup.elems.len() != 2 {
                return Err(Error::new(content.span(), "tuple must contain exactly two elements"));
            }
        }
        Ok(Self {
            punctuated: content,
        })
    }
}

#[proc_macro]
pub fn btree_proc(input: TokenStream) -> TokenStream {
    let btree_struct = parse_macro_input!(input as BTreeProcMacro);

    let insertions = btree_struct.punctuated.iter().map(|e| {
        let k = &e.elems[0];
        let v = &e.elems[1];
        quote! {
            b_tree.insert(#k, #v);
        }
    });

    let output = quote! {
        {
            let mut b_tree = BTreeMap::new();
            #(#insertions)*
            b_tree
        }
    };

    output.into()
}
