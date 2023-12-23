use heck::ToTitleCase;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{
    self,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token,
};
use syn::{DeriveInput, Expr};

pub fn eris_mob(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("C can only be derived for enums"),
    };

    let mut grade_sets = Vec::new();
    let mut alignment_sets = Vec::new();
    let mut vulnerability_sets = Vec::new();
    let mut actions_sets = Vec::new();
    let mut choices = vec![quote! { let mut choices = vec![]; }];
    for variant in variants {
        let variant_name = &variant.ident;
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("grade") {
                let grade: Expr = attr.parse_args().unwrap();
                choices.push(quote! { choices.extend(vec![(#name::#variant_name, #grade)]); });
                grade_sets.push(quote! { #name::#variant_name => #grade });
            }
            if attr.path().is_ident("alignment") {
                let alignment: Expr = attr.parse_args().unwrap();
                alignment_sets.push(quote! { #name::#variant_name => #alignment });
            }
            if attr.path().is_ident("vulnerability") {
                let vulnerability: Expr = attr.parse_args().unwrap();
                vulnerability_sets.push(quote! { #name::#variant_name => Some(#vulnerability) });
            }
            if attr.path().is_ident("actions") {
                let actions: Expr = attr.parse_args().unwrap();
                actions_sets.push(quote! { #name::#variant_name => #actions });
            }
        }
    }

    let q = quote! {
        impl rand::prelude::Distribution<Mob> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?core::marker::Sized>(&self, rng: &mut R) -> crate::enemy::Mob {
                #(#choices)*
                let maximum_weight = choices.iter().map(|&(_, grade)| grade as u32).max().unwrap();
                let weights: Vec<u32> = choices.iter().map(|&(mob, variant)| maximum_weight / variant as u32).collect();
                let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
                let chosen_index = dist.sample(&mut rand::thread_rng());
                choices[chosen_index].0
        }
    }
        use crate::EnemyEvents;
        impl crate::EnemyEvents for #name {
            fn grade(&self) -> crate::enemy::MobGrade {
                match self {
                    #(#grade_sets,)*
                }
            }
            fn actions(&self) -> Vec<crate::enemy::MobAction> {
                match self {
                    #(#actions_sets,)*
                }
            }
            fn alignment(&self) -> crate::unit::Alignment {
                match self {
                    #(#alignment_sets,)*
                }
            }
            fn vulnerability(&self) -> Option<crate::damage::DamageType> {
                match self {
                    #(#vulnerability_sets,)*
                    _ => None
                }
            }
        }
    };

    q.into()
}

struct VariantAttribute {
    // Note to self: when adding an attribute here, add it to #[proc_macro_derive]!
    grade: String,
    actions: Vec<String>,
    alignment: Option<String>,
    vulnerability: Option<String>,
}

impl Parse for VariantAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut grade: String = "Normal".to_string();
        let mut actions: Vec<String> = vec!["Bite".to_string()];
        let mut alignment = None;
        let mut vulnerability = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "grade" => {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;
                    grade = lit.value();
                }
                "actions" => {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;
                    // Expecting a comma seperated list of values
                    actions = lit
                        .value()
                        .split(",")
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();
                }
                "alignment" => {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;
                    alignment = Some(lit.value());
                }
                "vulnerability" => {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;
                    vulnerability = Some(lit.value());
                }

                _ => return Err(syn::Error::new(ident.span(), "unknown attribute")),
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        Ok(VariantAttribute {
            grade,
            actions,
            alignment,
            vulnerability,
        })
    }
}

fn parse_application_attributes(attrs: &[syn::Attribute]) -> Vec<VariantAttribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("mob"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<VariantAttribute, Token![,]>::parse_terminated)
                .unwrap()
        })
        .collect()
}

pub(crate) fn eris_flat_mob(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("ErisFlatMob can only be derived for enums"),
    };
    let mut choices = vec![quote! { let mut choices = vec![]; }];
    let mut grade_sets = Vec::new();
    let mut alignment_sets = Vec::new();
    let mut vulnerability_sets = Vec::new();
    let mut actions_sets = Vec::new();

    for variant in variants {
        let attrs = parse_application_attributes(&variant.attrs);
        let variant_name = &variant.ident;
        for attr in attrs {
            let grade = attr.grade.to_title_case();
            let grade = TokenStream2::from_str(&grade).unwrap();
            let alignment = attr.alignment;

            let vulnerability = attr.vulnerability;
            let actions: Vec<_> = attr
                .actions
                .iter()
                .map(|x| TokenStream2::from_str(&x).unwrap())
                .collect();

            choices.push(
                quote! { choices.extend(vec![(#name::#variant_name, crate::enemy::MobGrade::#grade)]); },
            );

            grade_sets.push(quote! { #name::#variant_name => #grade});

            if let Some(alignment) = alignment {
                let alignment = TokenStream2::from_str(&alignment).unwrap();
                alignment_sets.push(quote! { #name::#variant_name => #alignment });
            } else {
                alignment_sets.push(quote! { #name::#variant_name => Neutral });
            }

            if let Some(vulnerability) = vulnerability {
                let vulnerability = TokenStream2::from_str(&vulnerability).unwrap();
                vulnerability_sets.push(quote! { #name::#variant_name => Some(#vulnerability) });
            } else {
                vulnerability_sets.push(quote! { #name::#variant_name => None });
            }

            actions_sets.push(quote! { #name::#variant_name => vec![#(#actions),*] });
        }
    }

    let tokens = quote! {
        impl rand::prelude::Distribution<Mob> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?core::marker::Sized>(&self, rng: &mut R) -> crate::enemy::Mob {
                #(#choices)*
                let maximum_weight = choices.iter().map(|&(_, grade)| grade as u32).max().unwrap();
                let weights: Vec<u32> = choices.iter().map(|&(mob, variant)| maximum_weight / variant as u32).collect();
                let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
                let chosen_index = dist.sample(&mut rand::thread_rng());
                choices[chosen_index].0
        }
    }
        use crate::EnemyEvents;
        impl crate::EnemyEvents for #name {
            fn grade(&self) -> crate::enemy::MobGrade {
                use crate::enemy::MobGrade::*;
                match self {
                    #(#grade_sets,)*
                }
            }
            fn actions(&self) -> Vec<crate::enemy::MobAction> {
                use crate::skill::MobAction::*;
                match self {
                    #(#actions_sets,)*
                }
            }
            fn alignment(&self) -> crate::unit::Alignment {
                use crate::unit::Alignment::*;
                match self {
                    #(#alignment_sets,)*
                }
            }
            fn vulnerability(&self) -> Option<crate::damage::DamageType> {
                use crate::damage::DamageType::*;
                match self {
                    #(#vulnerability_sets,)*
                    _ => None
                }
            }
        }
    };
    tokens.into()
}
