mod items;
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

#[proc_macro_derive(ErisFlatMob, attributes(mob))]
#[proc_macro_error]
pub fn impl_arasaka_applications(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = mob_actions::eris_flat_mob(&ast);
    toks.into()
}
#[proc_macro_error]
#[proc_macro_derive(ErisItemTemplate, attributes(slot))]
pub fn eris_templates(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = items::eris_item_template(&ast);
    toks.into()
}

#[proc_macro_error]
#[proc_macro_derive(ErisConstructedTemplate)]
pub fn eris_constructed_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = items::eris_consturcted_template(&ast);
    toks.into()
}

#[proc_macro_error]
#[proc_macro_derive(ErisAssignEquipment)]
pub fn eris_equipment_assisgn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = items::eris_assign_equipment(&ast);
    toks.into()
}

#[proc_macro_error]
#[proc_macro_derive(ErisTryFrom)]
pub fn eris_try_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_valid_try(&ast);
    toks.into()
}
