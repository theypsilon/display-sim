extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[proc_macro_derive(StructIter, attributes(StructIterMethod))]
pub fn struct_iter(input: TokenStream) -> TokenStream {
    ImplContext::new(input, "StructIter", false).transform_ast()
}

struct ImplContext {
    ast: syn::DeriveInput,
    derive_name: &'static str,
    with_mutability: bool,
    ident_struct_iter_method : proc_macro2::Ident,
    ident_public : proc_macro2::Ident,
    ident_mutable : proc_macro2::Ident,
    ident_fields : proc_macro2::Ident,
}

lazy_static! {
}

impl ImplContext {
    fn new(input: TokenStream, derive_name: &'static str, with_mutability: bool) -> ImplContext {
        ImplContext {
            ast: syn::parse(input).expect("Could not parse AST."),
            derive_name,
            with_mutability,
            ident_struct_iter_method: proc_macro2::Ident::new("StructIterMethod", proc_macro2::Span::call_site()),
            ident_public: proc_macro2::Ident::new("public", proc_macro2::Span::call_site()),
            ident_mutable: proc_macro2::Ident::new("mutable", proc_macro2::Span::call_site()),
            ident_fields: proc_macro2::Ident::new("fields", proc_macro2::Span::call_site()),
        }
    }

    fn transform_ast(&self) -> TokenStream {
        let attrs = {
            let mut attrs = self.ast.attrs.clone();
            for attr in attrs.into_iter() {
                let is_struct_iter_method = attr.path.segments.iter().any(|segment| segment.ident == "StructIterMethod");
                if !is_struct_iter_method {
                    continue;
                }
                println!("tts: {:?}", attr.tts);
                for tts in attr.tts.into_iter() {
                    match tts {
                        TokenTree::Group(group) => {
                            println!("group: {:?}", group);
                            for tokens in group.stream().into_iter() {
                                println!("tokens: {:?}", tokens);
                            }
                        },
                        _ => {}
                    }
                }
            }
            ()
        };
        let fields_by_type = match self.ast.data {
            syn::Data::Struct(ref class) => self.read_fields(&class.fields),
            _ => panic!(
                "The type '{}' is not a struct but tries to derive '{}' which can only be used on structs.",
                self.ast.ident, self.derive_name
            ),
        };
        let mut methods = Vec::<TokenTree>::new();
        for (type_pieces, fields_sharing_type) in fields_by_type.into_iter() {
            let return_type = MethodReturnType {
                ty: fields_sharing_type.ty,
                name: make_type_name_from_type_pieces(type_pieces),
            };
            methods.extend(self.make_method_tokens("get_fields", &return_type, false, fields_sharing_type.immutable_fields));
            if self.with_mutability {
                methods.extend(self.make_method_tokens("get_mut_fields", &return_type, true, fields_sharing_type.mutable_fields));
            }
        }
        let (ty, generics) = (&self.ast.ident, &self.ast.generics);
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let tokens = quote! {
            impl #impl_generics #ty #ty_generics
                #where_clause
            {
                #(#methods)
                *
            }
        };
        tokens.into()
    }

    fn read_fields<'a>(&self, fields: &'a syn::Fields) -> HashMap<Vec<TypePart<'a>>, FieldsSharingType<'a>> {
        let mut fields_by_type = HashMap::<Vec<TypePart>, FieldsSharingType>::new();
        for field in fields.iter() {
            if field.attrs.is_empty() {
                continue;
            }
            for attr in field.attrs.iter() {
                println!("attr: {:?}", attr.tts);
            }
            if let Some(ref ident) = field.ident {
                let info = get_info_from_type(&field.ty);
                match make_idents_from_type(&field.ty) {
                    Ok(type_pieces) => {
                        let fields_by_type = fields_by_type.entry(type_pieces).or_insert_with(|| FieldsSharingType::new(info.ty));
                        if info.is_mutable && self.with_mutability {
                            fields_by_type.mutable_fields.push(ident);
                        }
                        fields_by_type.immutable_fields.push(ident);
                    }
                    Err(err) => {
                        eprintln!("[WARNING::{}] Field '{}' of struct '{}' not covered because: {}", self.derive_name, ident, self.ast.ident, err);
                    }
                }
            }
        }
        fields_by_type
    }

    fn make_method_tokens(&self, method_prefix: &str, return_type: &MethodReturnType, mutability: bool, field_idents: Vec<&syn::Ident>) -> proc_macro2::TokenStream {
        let count = field_idents.len();
        let method_name = syn::Ident::new(&format!("{}_{}", method_prefix, return_type.name), Span::call_site());
        let (vis, return_type) = (&self.ast.vis, &return_type.ty);
        if mutability {
            quote! {
                #vis fn #method_name(&mut self) -> [&mut #return_type; #count] {
                    [#(&mut self.#field_idents),*]
                }
            }
        } else {
            quote! {
                #vis fn #method_name(&self) -> [&#return_type; #count] {
                    [#(&self.#field_idents),*]
                }
            }
        }
    }
}

struct MethodReturnType<'a> {
    ty: &'a syn::Type,
    name: String,
}

struct FieldsSharingType<'a> {
    immutable_fields: Vec<&'a syn::Ident>,
    mutable_fields: Vec<&'a syn::Ident>,
    ty: &'a syn::Type,
}

impl<'a> FieldsSharingType<'a> {
    fn new(ty: &'a syn::Type) -> FieldsSharingType {
        FieldsSharingType {
            immutable_fields: vec![],
            mutable_fields: vec![],
            ty,
        }
    }
}

struct TypeInfo<'a> {
    is_mutable: bool,
    ty: &'a syn::Type,
}

#[derive(Hash, PartialEq, Eq)]
enum TypePart<'a> {
    Ident(&'a syn::Ident),
    Integer(u64),
    Separator(&'static str),
}

impl<'a> TypePart<'a> {
    fn to_string(&self) -> String {
        match self {
            TypePart::Ident(i) => i.to_string(),
            TypePart::Separator(s) => s.to_string(),
            TypePart::Integer(i) => i.to_string(),
        }
    }
}

fn get_info_from_type(ty: &syn::Type) -> TypeInfo {
    let (ty, is_mutable) = match ty {
        syn::Type::Reference(ref reference) => (&*reference.elem, reference.mutability.is_some()),
        _ => (ty, true),
    };
    TypeInfo { is_mutable, ty }
}

fn make_idents_from_type<'a>(ty: &'a syn::Type) -> Result<Vec<TypePart<'a>>, &'static str> {
    let mut type_pieces = Vec::<TypePart<'a>>::with_capacity(8);
    fill_type_pieces_from_type(&mut type_pieces, ty)?;
    Ok(type_pieces)
}

fn fill_type_pieces_from_type<'a>(type_pieces: &mut Vec<TypePart<'a>>, ty: &'a syn::Type) -> Result<(), &'static str> {
    match ty {
        syn::Type::Path(ref path) => fill_type_pieces_from_type_path(type_pieces, &path.path),
        syn::Type::Reference(ref reference) => fill_type_pieces_from_type(type_pieces, &reference.elem),
        syn::Type::BareFn(ref function) => {
            type_pieces.push(TypePart::Separator("fn("));
            fill_type_pieces_from_array_of_inputs(type_pieces, &function.inputs, ",", |type_pieces, arg| fill_type_pieces_from_type(type_pieces, &arg.ty))?;
            type_pieces.push(TypePart::Separator(")"));
            fill_type_pieces_from_return_type(type_pieces, &function.output)?;
            Ok(())
        }
        syn::Type::Slice(slice) => {
            type_pieces.push(TypePart::Separator("["));
            fill_type_pieces_from_type(type_pieces, &slice.elem)?;
            type_pieces.push(TypePart::Separator("]"));
            Ok(())
        }
        syn::Type::Array(array) => {
            type_pieces.push(TypePart::Separator("["));
            fill_type_pieces_from_type(type_pieces, &array.elem)?;
            type_pieces.push(TypePart::Separator(";"));
            match &array.len {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Int(int) => type_pieces.push(TypePart::Integer(int.value())),
                    _ => return Err("syn::Lit::* are not implemented yet."),
                },
                _ => return Err("syn::Expr::* are not implemented yet."),
            }
            type_pieces.push(TypePart::Separator("]"));
            Ok(())
        }
        syn::Type::Tuple(tuple) => {
            type_pieces.push(TypePart::Separator("("));
            fill_type_pieces_from_array_of_inputs(type_pieces, &tuple.elems, ",", fill_type_pieces_from_type)?;
            type_pieces.push(TypePart::Separator(")"));
            Ok(())
        }
        syn::Type::Paren(paren) => {
            type_pieces.push(TypePart::Separator("("));
            fill_type_pieces_from_type(type_pieces, &paren.elem)?;
            type_pieces.push(TypePart::Separator(")"));
            Ok(())
        }
        syn::Type::Ptr(ptr) => {
            type_pieces.push(TypePart::Separator("ptr_"));
            if ptr.const_token.is_some() {
                type_pieces.push(TypePart::Separator("const_"));
            }
            if ptr.mutability.is_some() {
                type_pieces.push(TypePart::Separator("mut_"));
            }
            fill_type_pieces_from_type(type_pieces, &ptr.elem)?;
            Ok(())
        }
        syn::Type::ImplTrait(_) => Err("syn::Type::ImplTrait can not be implemented."), // ImplTrait is not valid outside of functions and inherent return types, so can't be implemented.
        syn::Type::TraitObject(trait_object) => {
            if trait_object.dyn_token.is_some() {
                type_pieces.push(TypePart::Separator("dyn_"));
            }
            fill_type_pieces_from_array_of_inputs(type_pieces, &trait_object.bounds, "+", |type_pieces, bound| match bound {
                syn::TypeParamBound::Trait(trait_bound) => fill_type_pieces_from_type_path(type_pieces, &trait_bound.path),
                syn::TypeParamBound::Lifetime(_) => Ok(()),
            })
        }
        syn::Type::Never(_) => Err("syn::Type::Never is not implemented yet."),
        syn::Type::Group(_) => Err("syn::Type::Group is not implemented yet."),
        syn::Type::Infer(_) => Err("syn::Type::Infer is not implemented yet."),
        syn::Type::Macro(_) => Err("syn::Type::Macro is not implemented yet."),
        syn::Type::Verbatim(_) => Err("syn::Type::Verbatim is not implemented yet."),
    }
}

fn fill_type_pieces_from_type_path<'a>(type_pieces: &mut Vec<TypePart<'a>>, path: &'a syn::Path) -> Result<(), &'static str> {
    for segment in path.segments.iter() {
        type_pieces.push(TypePart::Ident(&segment.ident));
        fill_type_pieces_from_path_arguments(type_pieces, &segment.arguments)?;
    }
    Ok(())
}

fn fill_type_pieces_from_path_arguments<'a>(type_pieces: &mut Vec<TypePart<'a>>, arguments: &'a syn::PathArguments) -> Result<(), &'static str> {
    match arguments {
        syn::PathArguments::AngleBracketed(ref angle) => {
            type_pieces.push(TypePart::Separator("<"));
            fill_type_pieces_from_array_of_inputs(type_pieces, &angle.args, ",", |type_pieces, arg| match arg {
                syn::GenericArgument::Type(ref ty) => fill_type_pieces_from_type(type_pieces, ty),
                syn::GenericArgument::Lifetime(_) => Ok(()),
                syn::GenericArgument::Binding(_) => Ok(()),
                syn::GenericArgument::Constraint(_) => Ok(()),
                syn::GenericArgument::Const(_) => Ok(()),
            })?;
            type_pieces.push(TypePart::Separator(">"));
        }
        syn::PathArguments::None => {}
        syn::PathArguments::Parenthesized(ref paren) => {
            type_pieces.push(TypePart::Separator("("));
            fill_type_pieces_from_array_of_inputs(type_pieces, &paren.inputs, ",", fill_type_pieces_from_type)?;
            type_pieces.push(TypePart::Separator(")"));
            fill_type_pieces_from_return_type(type_pieces, &paren.output)?;
        }
    }
    Ok(())
}

fn fill_type_pieces_from_return_type<'a>(type_pieces: &mut Vec<TypePart<'a>>, output: &'a syn::ReturnType) -> Result<(), &'static str> {
    match output {
        syn::ReturnType::Default => Ok(()),
        syn::ReturnType::Type(_, ref arg) => fill_type_pieces_from_type(type_pieces, &**arg),
    }
}

fn fill_type_pieces_from_array_of_inputs<'a, T, U>(
    type_pieces: &mut Vec<TypePart<'a>>,
    inputs: &'a syn::punctuated::Punctuated<T, U>,
    separator: &'static str,
    action: impl Fn(&mut Vec<TypePart<'a>>, &'a T) -> Result<(), &'static str>,
) -> Result<(), &'static str> {
    if !inputs.is_empty() {
        for arg in inputs {
            action(type_pieces, arg)?;
            match type_pieces[type_pieces.len() - 1] {
                TypePart::Separator(_s) if _s == separator => {}
                _ => type_pieces.push(TypePart::Separator(separator)),
            }
        }
        match type_pieces[type_pieces.len() - 1] {
            TypePart::Separator(_s) if _s == separator => type_pieces.truncate(type_pieces.len() - 1),
            _ => {}
        }
    }
    Ok(())
}

fn make_type_name_from_type_pieces(type_pieces: Vec<TypePart>) -> String {
    type_pieces
        .into_iter()
        .map(|piece| piece.to_string())
        .collect::<String>()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '<' | '>' | '(' | ')' | '[' | ']' | '-' | ',' | ';' => '_',
            _ => c,
        })
        .collect()
}
