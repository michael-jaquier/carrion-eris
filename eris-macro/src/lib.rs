mod player_actions;

#[proc_macro_derive(ErisTitleCase)]
pub fn eris_title(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let toks = player_actions::eris_title_case(&ast);
    toks.into()
}

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
