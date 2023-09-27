use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{self};

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
                choices
                    .push(quote! { choices.extend(vec![#name::#variant_name; #grade as usize]); });
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
            fn sample<R: rand::Rng + ?core::marker::Sized>(&self, rng: &mut R) -> crate::enemies::Mob {
                #(#choices)*
                let index = rng.gen_range(0..choices.len());
                choices[index]
            }
        }
        use crate::EnemyEvents;
        impl crate::EnemyEvents for #name {
            fn grade(&self) -> crate::enemies::MobGrade {
                match self {
                    #(#grade_sets,)*
                }
            }
            fn actions(&self) -> Vec<crate::enemies::MobAction> {
                match self {
                    #(#actions_sets,)*
                }
            }
            fn alignment(&self) -> crate::units::Alignment {
                match self {
                    #(#alignment_sets,)*
                }
            }
            fn vulnerability(&self) -> Option<crate::units::DamageType> {
                match self {
                    #(#vulnerability_sets,)*
                    _ => None
                }
            }
        }
    };

    q.into()
}
