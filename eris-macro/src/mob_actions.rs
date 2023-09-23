use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site, ResultExt};
use quote::quote;
use syn::{
    self,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Token,
};

use syn::{DeriveInput, Expr};


struct MobAttributes {
    grade: syn::Expr,
    actions: syn::Expr,
    alignment: syn::Expr,
    vulnerability: syn::Expr,
}

impl Parse for MobAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        match input.parse::<Expr>() {
            Ok(expr) => {
                if name_str == "grade" {
                    Ok(MobAttributes {
                        grade: expr,
                        actions: syn::parse_quote! { 0 },
                        alignment: syn::parse_quote! { 0 },
                        vulnerability: syn::parse_quote! { 0 },
                    })
                } else if name_str == "actions" {
                    Ok(MobAttributes {
                        grade: syn::parse_quote! { 0 },
                        actions: expr,
                        alignment: syn::parse_quote! { 0 },
                        vulnerability: syn::parse_quote! { 0 },
                    })
                } else if name_str == "alignment" {
                    Ok(MobAttributes {
                        grade: syn::parse_quote! { 0 },
                        actions: syn::parse_quote! { 0 },
                        alignment: expr,
                        vulnerability: syn::parse_quote! { 0 },
                    })
                } else if name_str == "vulnerability" {
                    Ok(MobAttributes {
                        grade: syn::parse_quote! { 0 },
                        actions: syn::parse_quote! { 0 },
                        alignment: syn::parse_quote! { 0 },
                        vulnerability: expr,
                    })
                } else {
                    abort!(name, "Invalid attribute");
                }
            }
            Err(_) => abort!(name, "Invalid attribute"),
        }
    }
}

pub fn eris_mob(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mob_attributes: Vec<MobAttributes> = variants
        .iter()
        .map(|variant| {
            let mut grade = None;
            let mut actions = None;
            let mut alignment = None;
            let mut vulnerability = None;
            let attributes = &variant.attrs;
            for attr in variant.attrs.iter() {
                let sx = attr
                    .parse_args_with(
                        Punctuated::<MobAttributes, Token![,]>::parse_terminated)
                    .unwrap();
                for s in sx {
                    grade = Some(s.grade);
                    actions = Some(s.actions);
                    alignment = Some(s.alignment);
                    vulnerability = Some(s.vulnerability);
                }
            }
            MobAttributes {
                grade: grade.unwrap(),
                actions: actions.unwrap(),
                alignment: alignment.unwrap(),
                vulnerability: vulnerability.unwrap(),
            }
        })
        .collect();

    let mob_string: String = mob_attributes.iter().map(|m| {
        stringify!(m)
    }).collect::<Vec<_>>().join("\n");


    let q = quote! {

        use crate::EnemyEvents;
        impl crate::EnemyEvents for #name {
            fn grade(&self) -> crate::enemies::MobGrade {
                use #name::*;
                crate::enemies::MobGrade::Weak
            }
            fn actions(&self) -> Vec<crate::enemies::MobAction> {
                use #name::*;
               vec!
            }
            fn alignment(&self) -> crate::units::Alignment {
                use #name::*;
                match self {
                    #(#alignment_sets => #alignment.into(),)*
                }
            }
            fn vulnerability(&self) -> Option<crate::units::DamageType> {
                use #name::*;
                match self {
                    #(#vulnerability_sets => Some(#vulnerability),)*
                    _ => None
                }
            }
        }
    };

    q.into()
}
