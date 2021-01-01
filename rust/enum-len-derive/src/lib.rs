/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

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
