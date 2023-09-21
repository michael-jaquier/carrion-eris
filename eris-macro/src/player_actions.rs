use heck::*;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use syn::{parenthesized, DeriveInput, Expr};

pub fn eris_title_case(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("PlayerAction can only be derived for enums"),
    };
    let mut parents = vec![];
    let mut names = vec![];
    for variant in variants {
        parents.push(variant);
        names.push(variant.ident.to_string().to_title_case());
    }

    let q = quote! {
    pub fn names() {
        let names = vec![#(#names),*];
        let parents = vec![#(#parents),*];
    }

    impl std::fmt::Display for #name {
          fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", (format!("{:?}", self).to_title_case()))
            }
        }
    };

    q.into()
}

pub fn eris_from_dmg(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mut qq = vec![];
    let mut hit_enum = vec![];
    // Iterate through the attributes of each variant
    for variant in variants {
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("stat") {
                let stat_value: Expr = attr.parse_args().unwrap();
                hit_enum.push(variant.ident.clone());
                qq.push(stat_value);
            }
        }
    }

    let q = quote! {
        impl crate::AttributeScaling for #name {
            fn scaling(&self) -> Option<crate::units::Attribute> {
               use #name::*;
                match self {
                    #(#hit_enum => Some(#qq.into()),)*
                    _ => None,
                }

            }
        }
    };
    q.into()
}

pub fn eris_elemental_scaling(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };
    let mut qq = vec![];
    let mut hit_enum = vec![];
    // Iterate through the attributes of each variant
    for variant in variants {
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("element") {
                let stat_value: Expr = attr.parse_args().unwrap();
                hit_enum.push(variant.ident.clone());
                qq.push(stat_value);
            }
        }
    }

    let q = quote! {
        impl crate::ElementalScaling for #name {
            fn scaling(&self) -> Option<crate::units::DamageType> {
               use #name::*;
                match self {
                    #(#hit_enum => Some(#qq.into()),)*
                    _ => None,
                }

            }
        }
    };
    q.into()
}
