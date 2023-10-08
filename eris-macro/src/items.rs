use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Expr};

pub fn eris_assign_equipment(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Eris Item can only be derived for structs"),
    };

    let mut damage = vec![];
    let mut armor = vec![];
    for field in fields {
        let field_name = &field.ident;
        damage.push(
            quote! { if let Some(field) = &self.#field_name { base += field.generate().armor} },
        );
        armor.push(
            quote! { if let Some(field) = &self.#field_name { base += field.generate().damage} },
        );
    }

    let q = quote! {
        impl #name {
            pub fn damage(&self) -> Dice {
                 let mut base = Dice::zero();
                    #(#damage)*
                base
            }
            pub fn armor(&self) -> Dice {
                let mut base = Dice::zero();
                #(#armor)*
                base
            }
        }
    };
    q.into()
}

pub fn eris_consturcted_template(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("Eris Item can only be derived for enums"),
    };

    let mut f = vec![];
    let mut probability = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        f.push(quote! { #name::#variant_name => { #variant_name {}.generate()  } });
        probability
            .push(quote! { (#name::#variant_name, #variant_name {}.generate().rarity as u64) });
    }

    let q = quote! {
        impl #name {
            pub fn generate(&self) -> IndividualItem {
                match self {
                    #(#f,)*
                }
            }
             fn _prob() -> Option<#name> {
                use rand::Rng;
                let mut enum_weights = vec![#(#probability),*];

                // Calculate the total weight
                let total_weight: u64 = enum_weights.iter().map(|&(_, weight)| weight).sum();

                // Generate a random number between 0 and total_weight
                let mut rng = rand::thread_rng();
                let random_weight = rng.gen_range(0..total_weight);

                // Choose an enum based on the random weight
                let mut current_weight = 0;
                let mut selected_enum = None;
                for (my_enum, weight) in enum_weights {
                    current_weight += weight;
                    if random_weight < current_weight {
                        selected_enum = Some(my_enum);
                        break;
                    }
                }

                selected_enum
            }

            fn _prob_slot(slot: crate::items::EquipmentSlot) -> Option<#name> {

                use rand::Rng;
                let filtered_enum: Vec<_> = vec![#(#probability),*].into_iter().filter(|(item, _)| item.generate().slot == slot).collect();
                // Calculate the total weight
                let total_weight: u64 = filtered_enum.iter().map(|&(_, weight)| weight).sum();

                // Generate a random number between 0 and total_weight
                let mut rng = rand::thread_rng();
                let random_weight = rng.gen_range(0..total_weight);

                // Choose an enum based on the random weight
                let mut current_weight = 0;
                let mut selected_enum = None;
                for (my_enum, weight) in filtered_enum {
                    current_weight += weight;
                    if random_weight < current_weight {
                        selected_enum = Some(my_enum);
                        break;
                    }
                }

                selected_enum
            }

            pub fn generate_random() -> IndividualItem {
                let item = #name::_prob();
                item.expect("Unable to generate an item").generate()
            }

            pub fn generate_slot(slot: crate::items::EquipmentSlot) -> IndividualItem {
                let item = #name::_prob_slot(slot);
                item.expect("Unable to generate an item for slot").generate()
            }

            pub fn generate_random_item() -> Option<#name> {
                #name::_prob()
            }

            pub fn generate_random_item_slot(slot: crate::items::EquipmentSlot) -> #name {
                 let item = #name::_prob_slot(slot);
                item.expect("Unable to generate an item for slot")
            }




        }
    };

    q.into()
}

pub fn eris_item_template(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("Eris Item can only be derived for enums"),
    };

    let mut f = vec![];
    let mut probability = vec![];
    for variant in variants {
        let variant_name = &variant.ident;
        for attr in variant.attrs.iter() {
            if attr.path().is_ident("slot") {
                let slot: Expr = attr.parse_args().unwrap();
                f.push(quote! { #name::#variant_name => {let variant = #slot::#variant_name {}; return Box::new(variant);  } });
                probability.push(
                    quote! { (#name::#variant_name, #slot::#variant_name {}.rarity() as u64) },
                );
            }
        }
    }

    let q = quote! {
        impl #name {
            pub fn generate(&self) -> Box<dyn ItemProperties> {
                match self {
                    #(#f,)*
                }
            }

             fn _prob() -> Option<#name> {
                use rand::Rng;
                let mut enum_weights = vec![#(#probability),*];

                // Calculate the total weight
                let total_weight: u64 = enum_weights.iter().map(|&(_, weight)| weight).sum();

                // Generate a random number between 0 and total_weight
                let mut rng = rand::thread_rng();
                let random_weight = rng.gen_range(0..total_weight);

                // Choose an enum based on the random weight
                let mut current_weight = 0;
                let mut selected_enum = None;
                for (my_enum, weight) in enum_weights {
                    current_weight += weight;
                    if random_weight < current_weight {
                        selected_enum = Some(my_enum);
                        break;
                    }
                }

                selected_enum
            }

            fn _prob_slot(slot: crate::items::EquipmentSlot) -> Option<#name> {

                use rand::Rng;
                let filtered_enum: Vec<_> = vec![#(#probability),*].into_iter().filter(|(item, _)| item.generate().slot() == slot).collect();
                // Calculate the total weight
                let total_weight: u64 = filtered_enum.iter().map(|&(_, weight)| weight).sum();

                // Generate a random number between 0 and total_weight
                let mut rng = rand::thread_rng();
                let random_weight = rng.gen_range(0..total_weight);

                // Choose an enum based on the random weight
                let mut current_weight = 0;
                let mut selected_enum = None;
                for (my_enum, weight) in filtered_enum {
                    current_weight += weight;
                    if random_weight < current_weight {
                        selected_enum = Some(my_enum);
                        break;
                    }
                }

                selected_enum
            }

            pub fn generate_random() -> Box<dyn ItemProperties> {
                let item = #name::_prob();
                item.expect("Unable to generate an item").generate()
            }

            pub fn generate_slot(slot: crate::items::EquipmentSlot) -> Box<dyn ItemProperties> {
                let item = #name::_prob_slot(slot);
                item.expect("Unable to generate an item for slot").generate()
            }

            pub fn generate_random_item() -> Option<#name> {
                #name::_prob()
            }


        }
    };

    q.into()
}
