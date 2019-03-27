extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumLen)]
pub fn enum_len(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Could not parse AST.");
    let variants = match &ast.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("EnumLen can only be derived for enums"),
    };
    let count = variants.len();
    let (ty, generics) = (&ast.ident, &ast.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = quote! {
        impl #impl_generics enum_len_trait::EnumLen for #ty #ty_generics
            #where_clause
        {
            fn len() -> usize {
                #count
            }
        }
    };
    tokens.into()
}