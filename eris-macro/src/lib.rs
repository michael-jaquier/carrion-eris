mod mob_actions;
mod player_actions;
use proc_macro_error::proc_macro_error;
#[proc_macro_derive(AttributeScaling, attributes(stat))]
pub fn eris_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_from_dmg(&ast);
    toks.into()
}

#[proc_macro_derive(ElementalScaling, attributes(element))]
pub fn eris_elements(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_elemental_scaling(&ast);
    toks.into()
}

#[proc_macro_derive(ErisValidEnum, attributes(emoji))]
pub fn eris_valid_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_valid_enum(&ast);
    toks.into()
}

#[proc_macro_derive(ErisDisplayEmoji, attributes(emoji))]
pub fn eris_display_emoji(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_emoji(&ast);
    toks.into()
}

#[proc_macro_error]
#[proc_macro_derive(ErisMob, attributes(grade, alignment, vulnerability, actions))]
pub fn eris_mob(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = mob_actions::eris_mob(&ast);
    toks.into()
}
