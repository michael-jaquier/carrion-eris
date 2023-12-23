use heck::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use syn::{DeriveInput, Expr};

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
            fn scaling(&self) -> Option<String> {
               use #name::*;
                match self {
                    #(#hit_enum => Some(#qq.to_string()),)*
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
            fn scaling(&self) -> Option<crate::damage::DamageType> {
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

pub fn eris_emoji(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mut ee = vec![];
    // Iterate through the attributes of each variant
    for variant in variants {
        ee.push(variant.ident.clone());
    }

    let mut enum_string: HashMap<proc_macro2::Ident, String> = HashMap::new();
    for variant in variants {
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("emoji") {
                let stat_value: Expr = attr.parse_args().unwrap();
                let f = match stat_value {
                    Expr::Lit(lit) => {
                        let lit = lit.lit;
                        match lit {
                            syn::Lit::Str(lit_str) => lit_str.value(),
                            _ => panic!("emoji must be a string"),
                        }
                    }
                    _ => panic!("emoji must be a string"),
                };
                let mut str = String::new();
                str.push_str(&f);
                str.push_str(" ");
                str.push_str(&variant.ident.to_string().to_title_case());
                str.push_str(" ");
                str.push_str(&f);
                enum_string.insert(variant.ident.clone(), str.to_string());
            } else {
                if enum_string.contains_key(&variant.ident) {
                    continue;
                }
                enum_string.insert(
                    variant.ident.clone(),
                    variant.ident.to_string().to_title_case(),
                );
            }
        }

        if variant.attrs.is_empty() {
            if enum_string.contains_key(&variant.ident) {
                continue;
            }
            enum_string.insert(
                variant.ident.clone(),
                variant.ident.to_string().to_title_case(),
            );
        }
    }

    let key_vector = enum_string
        .iter()
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();
    let value_vector = enum_string
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<_>>();

    let mut q = quote! {
      impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use #name::*;
                match self {
                     #(#key_vector => write!(f, "{}", #value_vector),)*
                    _ => panic!("Unable to parse {} from {:?}", stringify!(#name), self)
                }
            }
        }
    };

    let try_from = eris_valid_try(ast);
    q.extend(try_from);

    q.into()
}

pub fn eris_valid_enum(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mut ee = vec![];
    // Iterate through the attributes of each variant
    for variant in variants {
        ee.push(variant.ident.clone());
    }

    let q = quote! {
        impl crate::ValidEnum for #name {
            fn valid() -> String {
                use #name::*;
                [#(#ee),*].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n")

            }

            fn valid_flat() -> String {
                use #name::*;
                [#(#ee),*].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("ø")
            }

        }
    };

    q.into()
}

pub fn eris_valid_try(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mut ee = vec![];
    // Iterate through the attributes of each variant
    for variant in variants {
        ee.push(variant.ident.clone());
    }

    let q = quote! {
        impl std::convert::TryFrom<String> for #name {
           type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                use #name::*;
                use heck::ToSnakeCase;
                let d = [#(#ee),*];
                let d_strings = d.iter().map(|x| x.to_string().to_snake_case()).collect::<Vec<String>>();
                let index = d_strings.iter().position(|x| x == &value.to_snake_case());

                match index {
                    Some(i) => Ok(d[i].clone()),
                    None => Err(format!("Unable to parse {} from {}", stringify!(#name), value))
                }

        }
    } };

    q.into()
}
